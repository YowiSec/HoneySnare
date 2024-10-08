// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Honeypot {
    event ExploitAttempt(address indexed bot, string action, uint256 amount, bool success);

    mapping(address => uint256) public balances;

    // Function that handles receiving Ether
    receive() external payable {
        balances[msg.sender] += msg.value;

        // Emit the ExploitAttempt event to indicate that the bot sent Ether
        emit ExploitAttempt(msg.sender, "sendEther", msg.value, false);  // Failed attempt
    }

    function transferOwnership() public {
        // Example exploit attempt
        emit ExploitAttempt(msg.sender, "transferOwnership", 0, false);  // Failed attempt
    }

    function withdraw(uint256 amount) public {
        // Example exploit attempt with an amount
        emit ExploitAttempt(msg.sender, "withdraw", amount, false);  // Failed attempt
    }

    function reentrancyExploit(uint256 amount) public {
        require(balances[msg.sender] >= amount, "Insufficient balance");

        balances[msg.sender] -= amount;  // Update state before sending funds

        (bool success, ) = msg.sender.call{value: amount}("");  // Simulate successful reentrancy

        if (success) {
            emit ExploitAttempt(msg.sender, "reentrancyExploit", amount, true);  // Success exploit attempt
        } else {
            balances[msg.sender] += amount;  // Revert balance deduction if the transfer fails
            emit ExploitAttempt(msg.sender, "reentrancyExploit", amount, false);  // Failed attempt
        }
    }

    function fakeVulnerability(uint256 amount) public {
        // Simulate some kind of internal calculation or manipulation
        if (balances[msg.sender] > amount) {
            // Pretend to allow a withdrawal
            emit ExploitAttempt(msg.sender, "fakeVulnerability", amount, true);  // Successful exploit attempt
        } else {
            emit ExploitAttempt(msg.sender, "fakeVulnerability", amount, false);  // Failed exploit attempt
        }
    }
}
