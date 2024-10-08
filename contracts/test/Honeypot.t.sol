// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/Honeypot.sol";

contract HoneypotTest is Test {
    Honeypot honeypot;

    function setUp() public {
        // Deploy the Honeypot contract
        honeypot = new Honeypot();
    }

    function testTransferOwnershipExploit() public {
        // Use a valid hexadecimal address for the bot
        address bot = address(0x1234567890123456789012345678901234567890);
        vm.prank(bot);  // Simulate the bot's address calling the function
        honeypot.transferOwnership();

        // Get recorded logs
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(logs.length, 1);  // Ensure one event was emitted

        // Assert that the exploit attempt was logged correctly
        assertEvent(logs[0], bot, "transferOwnership", 0, false);
    }

    function testWithdrawExploit() public {
        // Simulate a bot trying to withdraw an amount
        address bot = address(0x1234567890123456789012345678901234567890);
        vm.prank(bot);
        honeypot.withdraw(1 ether);

        // Get recorded logs
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(logs.length, 1);  // Ensure one event was emitted

        // Assert the exploit attempt was logged correctly
        assertEvent(logs[0], bot, "withdraw", 1 ether, false);
    }

    function testReentrancyExploit() public {
        // Deposit some funds into the contract
        (bool success, ) = address(honeypot).call{value: 1 ether}("");
        require(success, "Failed to send Ether");

        // Simulate a bot trying a reentrancy attack
        address bot = address(0x1234567890123456789012345678901234567890);
        vm.prank(bot);
        honeypot.reentrancyExploit(0.5 ether);

        // Get recorded logs
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(logs.length, 1);  // Ensure one event was emitted

        // Assert the reentrancy exploit was logged correctly
        assertEvent(logs[0], bot, "reentrancyExploit", 0.5 ether, true);  // Assuming the bot succeeds
    }

    function testFakeVulnerabilityExploit() public {
        // Simulate a bot triggering fake vulnerability
        address bot = address(0x1234567890123456789012345678901234567890);
        vm.prank(bot);
        honeypot.fakeVulnerability(1 ether);

        // Get recorded logs
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(logs.length, 1);  // Ensure one event was emitted

        // Assert the fake vulnerability exploit was logged correctly
        assertEvent(logs[0], bot, "fakeVulnerability", 1 ether, false);  // Assuming the bot fails
    }

    function testEtherSendToHoneypot() public {
        // Simulate a bot sending ether directly to the honeypot
        address bot = address(0x1234567890123456789012345678901234567890);
        vm.prank(bot);
        (bool success, ) = address(honeypot).call{value: 1 ether}("");
        assertTrue(success);

        // Get recorded logs
        Vm.Log[] memory logs = vm.getRecordedLogs();
        assertEq(logs.length, 1);  // Ensure one event was emitted

        // Assert the Ether send attempt was logged correctly
        assertEvent(logs[0], bot, "sendEther", 1 ether, false);  // Failed attempt since it's a honeypot
    }

    function assertEvent(
        Vm.Log memory log,
        address expectedBot,
        string memory expectedAction,
        uint256 expectedAmount,
        bool expectedSuccess
    ) internal {
        // Decode the log to check the event
        (address bot, string memory action, uint256 amount, bool success) = abi.decode(log.data, (address, string, uint256, bool));

        // Assert the values match what we expect
        assertEq(bot, expectedBot);
        assertEq(action, expectedAction);
        assertEq(amount, expectedAmount);
        assertEq(success, expectedSuccess);
    }
}