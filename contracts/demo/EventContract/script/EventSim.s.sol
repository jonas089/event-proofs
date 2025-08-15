// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {EventSim} from "../src/EventSim.sol";

contract EventScript is Script {
    EventSim public event_sim;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        event_sim = new EventSim();

        vm.stopBroadcast();
    }
}
