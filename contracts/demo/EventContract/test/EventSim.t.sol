// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {EventSim} from "../src/EventSim.sol";

contract EventTest is Test {
    EventSim public event_sim;

    function setUp() public {
        event_sim = new EventSim();
    }

    function testCall() public {
        event_sim.fire(1600, 0x9f2CD91d150236BA9796124F3Dcda305C3a2086C);
    }
}
