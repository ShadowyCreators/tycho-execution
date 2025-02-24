// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.26;

import "@src/executors/UniswapV4Executor.sol";

library UniswapV4Utils {
    function encodeExactInput(
        address tokenIn,
        address tokenOut,
        bool zeroForOne,
        address callbackExecutor,
        UniswapV4Executor.UniswapV4Pool[] memory pools
    ) public pure returns (bytes memory) {
        bytes memory encodedPools;

        for (uint256 i = 0; i < pools.length; i++) {
            encodedPools = abi.encodePacked(
                encodedPools,
                pools[i].intermediaryToken,
                bytes3(pools[i].fee),
                pools[i].tickSpacing
            );
        }

        return abi.encodePacked(
            tokenIn, tokenOut, zeroForOne, callbackExecutor, encodedPools
        );
    }
}
