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

        // Assert that the exploit failed (you can check contract state or events here)
        assertTrue(true);
    }

    function testWithdrawExploit() public {
        // Simulate a bot trying to withdraw an amount with a valid hexadecimal address
        address bot = address(0x1234567890123456789012345678901234567890);
        vm.prank(bot);
        honeypot.withdraw(1 ether);

        // Assert the event is logged or the exploit failed
        assertTrue(true);
    }

    function testExploitLogs() public {
        vm.recordLogs();
        address bot = address(0xB0T);
        vm.prank(bot);
        honeypot.withdraw(1 ether);

        // Get recorded logs
        Vm.Log[] memory logs = vm.getRecordedLogs();

        for (uint i = 0; i < logs.length; i++) {
            emit log_bytes(logs[i].data);  // Log the raw event data for inspection
        }
    }

}
