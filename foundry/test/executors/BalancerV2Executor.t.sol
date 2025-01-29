// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

import "@src/executors/BalancerV2Executor.sol";
import {Test} from "../../lib/forge-std/src/Test.sol";
import {Constants} from "../Constants.sol";

contract BalancerV2ExecutorExposed is BalancerV2Executor {
    function decodeParams(bytes calldata data)
        external
        pure
        returns (
            IERC20 tokenIn,
            IERC20 tokenOut,
            bytes32 poolId,
            address receiver,
            bool needsApproval
        )
    {
        return _decodeData(data);
    }
}

contract BalancerV2ExecutorTest is
    BalancerV2ExecutorExposed,
    Test,
    Constants
{
    using SafeERC20 for IERC20;

    BalancerV2ExecutorExposed balancerV2Exposed;
    IERC20 WETH = IERC20(WETH_ADDR);
    IERC20 BAL = IERC20(BAL_ADDR);
    bytes32 constant WETH_BAL_POOL_ID =
        0x5c6ee304399dbdb9c8ef030ab642b10820db8f56000200000000000000000014;

    function setUp() public {
        uint256 forkBlock = 17323404;
        vm.createSelectFork(vm.rpcUrl("mainnet"), forkBlock);
        balancerV2Exposed = new BalancerV2ExecutorExposed();
    }

    function testDecodeParams() public view {
        bytes memory params = abi.encodePacked(
            WETH_ADDR, BAL_ADDR, WETH_BAL_POOL_ID, address(2), true
        );

        (
            IERC20 tokenIn,
            IERC20 tokenOut,
            bytes32 poolId,
            address receiver,
            bool needsApproval
        ) = balancerV2Exposed.decodeParams(params);

        assertEq(address(tokenIn), WETH_ADDR);
        assertEq(address(tokenOut), BAL_ADDR);
        assertEq(poolId, WETH_BAL_POOL_ID);
        assertEq(receiver, address(2));
        assertEq(needsApproval, true);
    }

    function testDecodeParamsInvalidDataLength() public {
        bytes memory invalidParams =
            abi.encodePacked(WETH_ADDR, BAL_ADDR, WETH_BAL_POOL_ID, address(2));

        vm.expectRevert(BalancerV2Executor__InvalidDataLength.selector);
        balancerV2Exposed.decodeParams(invalidParams);
    }

    function testSwap() public {
        uint256 amountIn = 10 ** 18;
        bytes memory protocolData =
            abi.encodePacked(WETH_ADDR, BAL_ADDR, WETH_BAL_POOL_ID, BOB, true);

        deal(WETH_ADDR, address(balancerV2Exposed), amountIn);
        uint256 balanceBefore = BAL.balanceOf(BOB);

        uint256 amountOut = balancerV2Exposed.swap(amountIn, protocolData);

        uint256 balanceAfter = BAL.balanceOf(BOB);
        assertGt(balanceAfter, balanceBefore);
        assertEq(balanceAfter - balanceBefore, amountOut);
    }
}
