// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/TokenLiquidityManager.sol";

contract DeployScript is Script {
    function run() external {
        // Get deployer info
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);
        
        // Log pre-deployment info
        console.log("Deploying from address:", deployer);
        console.log("Deployer balance:", deployer.balance);
        
        vm.startBroadcast(deployerPrivateKey);

        // Deploy with logs
        console.log("Starting deployment...");
        TokenLiquidityManager honeypot = new TokenLiquidityManager();
        console.log("Deployment transaction sent!");
        console.log("Honeypot deployed to:", address(honeypot));
        console.log("Remaining balance:", deployer.balance);

        vm.stopBroadcast();
        
        // Log post-deployment info
        console.log("Deployment completed!");
        console.log("Verify contract at:", address(honeypot));
        console.log("Transaction should appear on Arbiscan shortly");
    }
}