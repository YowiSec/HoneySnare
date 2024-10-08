// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Honeypot {
    event ExploitAttempt(address indexed bot, string action, uint256 amount, bool success);

    function transferOwnership() public {
        // Example exploit attempt
        emit ExploitAttempt(msg.sender, "transferOwnership", 0, false);  // Failed attempt
    }

    function withdraw(uint256 amount) public {
        // Example exploit attempt with an amount
        emit ExploitAttempt(msg.sender, "withdraw", amount, false);  // Failed attempt
    }
}
