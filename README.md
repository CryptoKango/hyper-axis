# HyperAxis

HyperAxis 是面向 Hyperliquid 客户端的 Rust 协议核心库。移动端可以通过
FFI 集成，网页端可以通过 WebAssembly 集成，从而让多个客户端共享同一套
精度校验、协议编码、哈希和签名摘要逻辑。

本仓库只存放 Rust 相关代码。Dart 和 TypeScript 封装位于各自独立的仓库中，
通过 FFI 或 WebAssembly 使用本仓库生成的产物。

## 当前能力

- 标准化十进制字符串，避免浮点数造成金额精度丢失；
- Hyperliquid 价格精度、数量精度和有效数字校验；
- 普通永续合约、HIP-3 永续合约和现货的 Asset ID 计算；
- 与 Hyperliquid 线协议兼容的订单 Action 数据结构；
- 原子批量下单、按 OID/Cloid 批量撤单和批量改单；
- 混合限价/触发订单、计划撤单（dead man's switch）和原生 TWAP；
- 永续合约 cross/isolated 杠杆设置、隔离保证金增减和 isolated-only 保证金补充；
- Core USDC/Spot 转账与桥接提现的独立 User-Signed EIP-712 管线；
- API Wallet 与 Builder Fee 授权的独立 User-Signed EIP-712 管线；
- Spot/Perp USD 划转与跨用户、子账户、DEX 的通用资产转账；
- 账户抽象模式切换、旧版 DEX 抽象开关与多签账户转换；
- 共识签名节点 jail/unjail、验证者注册/资料变更/注销与 gossip priority bid；
- Spot token/market 完整部署生命周期与 HIP-3 perpetual asset/oracle 部署；
- 128-bit Cloid 和 EVM 地址强类型解析，拒绝非规范边界值；
- Action 的 MessagePack 编码和 Keccak 哈希计算；
- L1 Phantom Agent 的 EIP-712 Domain、消息结构和待签摘要计算；
- 从订单参数到 Action、Action Hash、待签摘要的统一编排接口；
- secp256k1 签名格式校验、Low-S 校验、公钥恢复和签名账户核对；
- 最终 `/exchange` 请求体生成及签名前后完整性复验；
- 按签名账户隔离的并发安全 Nonce 管理与时间窗口预检；
- 运行时可开关的跨平台结构化诊断日志；
- 独立于网络运行时的 HTTP/WebSocket 请求模型、响应解析和重试分类；
- 官方 Python SDK 全部 38 个 Info 查询方法（37 种 wire type）的强类型请求、校验与 HTTP/WS 编排；
- 官方 24 类 WebSocket 订阅、心跳、订阅确认及 `post` 请求封装；
- 面向 Flutter/Dart 的 FFI 接口层；
- 面向浏览器和 TypeScript 的 WebAssembly 接口层。

## 安全边界

HyperAxis 不保存、读取或托管用户私钥。Rust Core 只负责生成经过协议校验的
Action、Action Hash 和 EIP-712 待签摘要。私钥安全存储和 secp256k1 签名由
App、浏览器钱包或硬件钱包等宿主环境负责。宿主返回签名后，Core 会验证 `r/s/v`、
拒绝 High-S 签名、恢复签名地址并与预期账户核对，再生成最终请求体。

金额、价格、数量和费率等数据必须以十进制字符串传入，禁止先转换为
`double`、JavaScript `Number` 或其他可能丢失精度的类型。Nonce 和过期时间在
WebAssembly 边界同样使用字符串传递，避免 JavaScript 64 位整数精度问题。

## Workspace 结构

| Crate | 职责 |
| --- | --- |
| `hyper-axis-types` | 公共协议类型、网络类型和统一错误模型 |
| `hyper-axis-decimal` | 无损十进制解析、标准化和精度信息 |
| `hyper-axis-logging` | 跨平台结构化日志、级别过滤和环形缓冲区 |
| `hyper-axis-nonce` | 按签名账户隔离的原子 Nonce 协调与时间窗口预检 |
| `hyper-axis-assets` | Asset ID 计算和市场精度校验 |
| `hyper-axis-actions` | 强类型交易 Action 模型 |
| `hyper-axis-codec` | MessagePack 编码和 Action Hash 计算 |
| `hyper-axis-signing` | EIP-712 摘要、签名规范化、公钥恢复和账户校验 |
| `hyper-axis-client` | 订单准备、签名后复验与最终 Exchange 请求编排 |
| `hyper-axis-transport` | HTTP/WebSocket 协议、响应模型和保守重试语义，不执行网络 I/O |
| `hyper-axis-core` | 面向所有原生消费者的稳定公共入口 |
| `hyper-axis-ffi` | 面向 Flutter/Dart 绑定的原生接口 |
| `hyper-axis-wasm` | 面向浏览器和 TypeScript 的 WASM 导出接口 |

各 crate 的 `lib.rs` 仅保留 crate 级文档、共享依赖、公共常量和源码装配，具体实现按职责拆分：

- `hyper-axis-client`：`models`、`prepare`、`multisig`、`finalize`、`internal`；
- `hyper-axis-actions`：`orders`、`deploy`、`validators`、`user`、`validation`；
- `hyper-axis-signing`：`models`、`l1`、`user`、`multisig`、`eip712`；
- `hyper-axis-transport`：`info`、`http`、`websocket`、`error`；
- `hyper-axis-ffi` / `hyper-axis-wasm`：`runtime`、`transport`、`prepare`、`multisig`、`finalize`、平台内部辅助实现；
- 基础 crate 同样按网络、标识符、精度、编码、日志状态等单一职责拆分；测试统一放在相邻的 `tests.rs`（Info 查询使用 `info_tests.rs`）。

为了保持已经公开的 Rust、FFI 和 WASM API 路径不变，功能文件由 crate 根按固定顺序装配；调用方无需修改 import。

## 协议一致性

`test-vectors/` 存放跨语言黄金测试向量。下单、撤单、改单、杠杆和保证金向量使用 Python 的
`msgpack.packb`、Keccak 和 `eth-account` EIP-712 编码流程生成；账户与多签向量也由
`eth-account` 独立生成，用于验证 Rust、Dart 和 TypeScript 实现输出完全一致。

协议字段顺序、MessagePack 编码、Nonce、Vault 标记、过期时间以及 EIP-712
Domain 都会影响最终签名结果。修改相关代码时必须同步更新或验证黄金向量。

## 下单调用链

一次完整的限价单请求分为以下边界：

1. App/Web 从官方元数据取得 Asset ID 与 `szDecimals`，金额字段始终保留为字符串；
2. 每个 master account 或 API Wallet 持有一个独立的 `NonceManager`；
3. 调用 `prepare_limit_order` / `prepare_limit_order_json` 完成精度校验、Action 编码、
   Action Hash 和 EIP-712 摘要计算；
4. 宿主钱包只签名返回的 32 字节摘要，不再做第二次哈希；
5. 调用 `finalize_order` / `finalize_order_json`，Core 会重新计算全部中间产物，验证
   签名账户后输出 endpoint 和最终 JSON；
6. 调用 `build_exchange_http_request_json` 生成宿主可执行的 HTTP 描述，App/Web
   使用自己的 Dio、Fetch 或受控网络栈发送其中的原始 `body`。Core 不会自行广播
   订单，也不会保存认证信息或私钥。

`PreparedOrder` 是可序列化的中间产物，但不能被视为可信输入。最终化阶段会从强类型
Action 重新生成 `actionJson`、MessagePack、Action Hash 和签名摘要，任何字段被篡改
都会失败。最终请求固定包含 `action`、`nonce`、`signature`、`vaultAddress` 和
`expiresAfter`；两个可选字段未设置时仍序列化为 `null`，与官方 SDK 请求形状一致。

交易动作已经共用泛型 `PreparedAction<A>` 和同一套最终化管线，避免不同 Action
各自实现哈希、签名和完整性校验而产生漂移。目前公开的准备/最终化接口包括：

| 动作 | Rust/FFI 准备接口 | Web 准备接口 |
| --- | --- | --- |
| 单笔限价单 | `prepare_limit_order` | `prepare_limit_order_json` |
| 原子批量限价单 | `prepare_batch_orders` | `prepare_batch_orders_json` |
| 按 OID 批量撤单 | `prepare_cancel` | `prepare_cancel_json` |
| 按 Cloid 批量撤单 | `prepare_cancel_by_cloid` | `prepare_cancel_by_cloid_json` |
| 批量改单 | `prepare_batch_modify` | `prepare_batch_modify_json` |
| 混合限价/触发单 | `prepare_orders` | `prepare_orders_json` |
| 设置/清除计划撤单 | `prepare_schedule_cancel` | `prepare_schedule_cancel_json` |
| 原生 TWAP 下单 | `prepare_twap_order` | `prepare_twap_order_json` |
| 原生 TWAP 撤单 | `prepare_twap_cancel` | `prepare_twap_cancel_json` |
| 设置 cross/isolated 杠杆 | `prepare_update_leverage` | `prepare_update_leverage_json` |
| 增减隔离保证金 | `prepare_update_isolated_margin` | `prepare_update_isolated_margin_json` |
| isolated-only 保证金补充 | `prepare_top_up_isolated_only_margin` | `prepare_top_up_isolated_only_margin_json` |
| 创建子账户 | `prepare_create_sub_account` | `prepare_create_sub_account_json` |
| 主账户/子账户 USD 划转 | `prepare_sub_account_transfer` | `prepare_sub_account_transfer_json` |
| 主账户/子账户 Spot 划转 | `prepare_sub_account_spot_transfer` | `prepare_sub_account_spot_transfer_json` |
| Vault USD 存取 | `prepare_vault_transfer` | `prepare_vault_transfer_json` |
| 设置推荐码 | `prepare_set_referrer` | `prepare_set_referrer_json` |
| EVM 大区块开关 | `prepare_evm_user_modify` | `prepare_evm_user_modify_json` |
| Agent 旧版 DEX 抽象启用 | `prepare_agent_enable_dex_abstraction` | `prepare_agent_enable_dex_abstraction_json` |
| Agent 账户抽象模式 | `prepare_agent_set_abstraction` | `prepare_agent_set_abstraction_json` |
| 空 L1 动作 | `prepare_noop` | `prepare_noop_json` |
| Spot 部署器（10 个变体） | `prepare_spot_deploy` | `prepare_spot_deploy_json` |
| Perp 部署器（2 个变体） | `prepare_perp_deploy` | `prepare_perp_deploy_json` |
| Core USDC 转账 | `prepare_usd_send` | `prepare_usd_send_json` |
| Core Spot 资产转账 | `prepare_spot_send` | `prepare_spot_send_json` |
| 桥接提现 | `prepare_withdraw` | `prepare_withdraw_json` |
| 授权 API Wallet | `prepare_approve_agent` | `prepare_approve_agent_json` |
| 授权 Builder Fee | `prepare_approve_builder_fee` | `prepare_approve_builder_fee_json` |
| Spot/Perp USD 划转 | `prepare_usd_class_transfer` | `prepare_usd_class_transfer_json` |
| 通用资产转账 | `prepare_send_asset` | `prepare_send_asset_json` |
| Spot → Staking | `prepare_c_deposit` | `prepare_c_deposit_json` |
| Staking → Spot | `prepare_c_withdraw` | `prepare_c_withdraw_json` |
| 验证者委托/解除委托 | `prepare_token_delegate` | `prepare_token_delegate_json` |
| 旧版 DEX 抽象开关 | `prepare_user_dex_abstraction` | `prepare_user_dex_abstraction_json` |
| 设置账户抽象模式 | `prepare_user_set_abstraction` | `prepare_user_set_abstraction_json` |
| 转换为多签用户 | `prepare_convert_to_multi_sig_user` | `prepare_convert_to_multi_sig_user_json` |

改单即使只有一笔也通过 `batchModify` 提交，与官方 Python SDK 的实现保持一致。每种
动作均有对应的 `finalize_*` 接口；最终化之前会重新计算并核对所有中间产物。

## 跨语言整数规则

JavaScript 无法无损表达全部 `u64`。因此 Web 请求 DTO 中的 `nonce`、
`expiresAfter`、OID、计划撤单时间、TWAP ID 和质押 `wei` 必须使用十进制字符串，例如
`"1234567890000"`。传入 JSON Number 会直接被拒绝，防止数据已经发生精度损失后
再进入 Rust。Asset ID、`szDecimals`、TWAP 分钟数、builder fee 和签名 `v` 的范围
较小，仍使用 JSON Number。

隔离保证金 `usdDelta` 使用带符号十进制字符串，Rust 会无舍入地转换为协议要求的
6 位 USD 整数 `ntli`。超过 6 位小数、结果为零或超出 `i64` 范围都会在签名前拒绝。
`updateLeverage` 的 `maxLeverage` 必须来自当前 perpetual metadata，Core 会校验目标
整数杠杆位于 `1..=maxLeverage`，但服务端仍是市场状态的最终权威。

Hyperliquid 当前没有独立的 trailing-stop L1 Action。追踪止损需要 App/服务端持续读取
行情并通过改单或触发单维护，属于有在线可用性和竞态处理要求的策略层，不能伪装成
一次链上原子操作。本仓库目前只提供构建该策略所需的触发单、改单和行情订阅协议能力。

`createSubAccount`、`subAccountTransfer`、`subAccountSpotTransfer` 和 `vaultTransfer` 使用
L1 Phantom Agent 签名。按照官方 SDK，这些 Action 的外层签名 `vaultAddress` 固定为空：
目标 subaccount/vault 地址属于 Action 本身，不能同时写入外层 vault 上下文。它们支持
`expiresAfter`。USD 输入使用精确十进制字符串，由 Core 无舍入转换为 6 位协议整数；
零值、负数、超过 6 位小数和 `u64` 溢出都会在签名前拒绝。Spot 划转金额保持规范十进制
字符串，同时校验 `name:identifier` token 形状。账户归属、余额、名称唯一性和服务端限额
仍由 Hyperliquid 最终判定。

`setReferrer`、`evmUserModify`、`agentEnableDexAbstraction`、`agentSetAbstraction` 和 `noop`
均通过 L1 Phantom Agent 管线构造。按照官方 SDK，推荐码和 EVM 大区块开关的外层
`vaultAddress` 固定为空；两个 Agent 抽象动作允许使用 signer 对应的 vault/subaccount
上下文；`noop` 保留可选签名上下文。Agent 模式使用 L1 wire 值 `i`、`u`、`p`，分别对应
disabled、unified account 和 portfolio margin，不能与 User-Signed 动作使用的长字符串混用。
`noop` 只提供协议兼容的空动作，不应被业务层当作网络探活或交易成功证明。

## User-Signed 转账、提现与授权

`usdSend`、`spotSend` 和 `withdraw3` 不使用 L1 Phantom Agent 摘要，而是分别使用
`HyperliquidTransaction:UsdSend`、`HyperliquidTransaction:SpotSend` 和
`HyperliquidTransaction:Withdraw` EIP-712 primary type。Domain 固定为：

- `name = HyperliquidSignTransaction`；
- `version = 1`；
- `chainId = 0x66eee`；
- `verifyingContract = 0x0000000000000000000000000000000000000000`。

`hyperliquidChain` 使用 `Mainnet`/`Testnet` 防止跨环境重放。调用方只传一个字符串
`nonce`，Core 会同时把它写入 action 的 `time`/`nonce` 和外层请求 `nonce`，不允许
两者独立输入。最终请求只包含 `action`、`nonce`、`signature`，不包含 L1 Action 使用的
`vaultAddress` 或 `expiresAfter`。

用户签名的 prepared JSON 同样必须作为不透明 token 保存。最终化阶段会重新生成 action
JSON 与 EIP-712 摘要，核对 network、`hyperliquidChain`、内部 `time`/`nonce` 和外层 nonce，
然后恢复签名地址并与预期账户比较。金额必须是正数十进制字符串；Spot token 必须使用
`name:identifier` 格式。HyperAxis 只构造和验证请求，不替用户判断目标地址或执行真实转账。

API Wallet 授权使用 `HyperliquidTransaction:ApproveAgent`。调用方必须在宿主安全环境中
自行生成 API Wallet 密钥，只把 `agentAddress` 传给 HyperAxis；Rust Core 不生成、不接收、
不返回其私钥。具名授权会发送 `agentName`；无名授权最终 Action 省略该字段，但签名摘要仍按
官方规则将空字符串纳入 EIP-712，不能直接删除签名字段。

Builder Fee 授权使用 `HyperliquidTransaction:ApproveBuilderFee`。`maxFeeRate` 必须是精确
百分比字符串（例如 `"0.001%"`），全程不经过浮点数。Core 会拒绝缺少 `%`、包含空白、
负数或指数格式的费率，但服务端仍负责账户权限、零费率语义及市场上限校验。

`usdClassTransfer` 使用 `HyperliquidTransaction:UsdClassTransfer`，用于 Spot USDC 与
默认 Perp USDC 余额之间划转。`toPerp=true` 表示 Spot → Perp，`false` 表示 Perp → Spot。
子账户场景的官方协议会把地址拼进签名金额，例如
`"12.5 subaccount:0x..."`。调用方只传独立的 `amount` 和可选 `subAccount`，Core 负责
校验地址并构造组合字符串，禁止宿主自行拼接。

`sendAsset` 使用 `HyperliquidTransaction:SendAsset`，可在用户、子账户、Spot、默认 Perp
DEX 和 HIP-3 Perp DEX 之间转移资产。DEX 名 `""` 表示默认 Perp，`"spot"` 表示现货；
主账户来源会被编码为必填的 `fromSubAccount: ""`，子账户来源则编码为规范 EVM 地址。
Core 校验地址、正数金额、token 符号/`name:identifier` 形状和 DEX 字符串边界；只有抵押品
可以进出 Perp DEX 等依赖实时元数据的业务约束仍由服务端最终判定。

`cDeposit`、`cWithdraw` 和 `tokenDelegate` 也属于 User-Signed 动作，分别使用
`HyperliquidTransaction:CDeposit`、`HyperliquidTransaction:CWithdraw` 和
`HyperliquidTransaction:TokenDelegate`。它们不允许 `vaultAddress`/`expiresAfter`，内部
`nonce` 必须与外层 nonce 相同。`wei` 是协议最小单位的正 `uint64`；跨语言准备请求必须
使用十进制字符串，Core 会在生成签名摘要前拒绝零值和溢出。`tokenDelegate` 还会规范化并
校验 validator 地址，`isUndelegate=false` 表示委托，`true` 表示解除委托。协议规定
Staking → Spot 经过 7 天 unstaking queue，针对同一 validator 的委托有 1 天锁定期；
这些是服务端状态规则，Core 只负责构造、签名上下文校验和最终请求完整性验证。

账户模式相关动作同样走 User-Signed EIP-712 管线：`userDexAbstraction` 是兼容旧协议的
布尔开关；新接口 `userSetAbstraction` 接受 `disabled`、`unifiedAccount`、
`portfolioMargin` 三种官方 wire 值。调用方传入目标 `user` 地址，Core 负责地址规范化、
内部 nonce 与外层 nonce 一致性及签名前后完整性复验。账户是否具备切换资格、当前持仓能否
切换模式等实时业务约束由 Hyperliquid 服务端最终判定。

`convertToMultiSigUser` 的 `signers` 在官方协议中不是嵌套对象，而是参与 EIP-712 的规范
JSON 字符串。为避免 Dart/TypeScript 的对象顺序或空格差异改变摘要，宿主只传
`authorizedUsers` 和 `threshold`：Core 解析并按地址字节升序排列，拒绝空集合、重复地址、
零阈值和超过授权人数的阈值，再生成官方 SDK 兼容的字符串。此操作会改变账户授权模型，
接入层应在签名前给出强风险确认；转换资格与后续多签执行规则仍以服务端为准。

## MultiSig 两阶段签名

当前已经实现官方 `multiSig` 外层签名聚合协议。L1 inner action 已覆盖 `order`、`cancel`、
`cancelByCloid`、`batchModify`、`scheduleCancel`、`twapOrder`、`twapCancel`、
`updateLeverage`、`updateIsolatedMargin`、`topUpIsolatedOnlyMargin`、`createSubAccount`、
`subAccountTransfer`、`subAccountSpotTransfer`、`vaultTransfer`、`setReferrer`、
`evmUserModify`、`agentEnableDexAbstraction`、`agentSetAbstraction`、`noop`、
`CSignerAction`、`CValidatorAction`、`gossipPriorityBid`、`spotDeploy` 和 `perpDeploy`；
其中部署器聚合入口覆盖 10 个 Spot 变体与 2 个 Perp 变体。User-Signed inner action 已覆盖
`usdSend`、`spotSend`、`withdraw3`、`approveAgent`、`approveBuilderFee`、
`usdClassTransfer`、`sendAsset`、`cDeposit`、`cWithdraw`、`tokenDelegate`、
`userDexAbstraction` 和 `userSetAbstraction`。完整调用链是：

1. 先通过普通 `prepare_*` 接口生成并校验原始 Action；
2. 调用 `prepare_multi_sig_*_payload`，生成所有 authorized user 共同签名的 inner digest；
3. 宿主分别收集 `{expectedSigner,r,s,v}`，调用 `prepare_multi_sig_envelope`；
4. Core 验证每个签名、拒绝重复 signer，并按恢复地址排序后生成 `SendMultiSig` outer digest；
5. outer signer 签名后调用 `finalize_multi_sig`，Core 重算全部 inner/outer 产物并生成最终请求。

Web 端使用对应的 `*_json` 接口，所有 prepared payload/envelope 都必须视为不透明 token。
`userSetAbstraction` 的 authorized users 签名人类可读长值，但 outer payload 会按官方协议改写为
`i/u/p`；两份内容不同是协议要求。Core 会验证声明 signer 与恢复地址、签名唯一性和规范顺序，
但 authorized-user 成员关系、账户阈值和 outer signer 权限仍由 Hyperliquid 服务端最终判断。
独立向量覆盖上述每种 inner action 的 inner digest、outer MessagePack、action hash、
`SendMultiSig` digest 和 authorized-user 签名恢复，并覆盖代表性动作的 outer 签名恢复及最终请求。
`convertToMultiSigUser` 不作为已完成转换后的常规 inner action 暴露；账户模式迁移资格与
多签账户恢复流程应继续以 Hyperliquid 服务端规则为准。

`PreparedAction` 顶层的 `nonce`/`expiresAfter` 会序列化成字符串，但其内部 Action
必须保留 Hyperliquid 的真实线协议类型，其中 OID、计划撤单时间和 TWAP ID 仍是
JSON Number。因此 Web 端必须把 `prepare_*_json` 返回的完整字符串视为不透明 token，
不得 `JSON.parse` 后重新序列化。签名时调用 `prepared_signing_digest(token)` 读取摘要，
完成签名后把原 token 原样传给对应的 `finalize_*_json`。最终得到的 `requestJson`
也应作为原始 HTTP body 直接发送。

## Nonce 管理

Hyperliquid 按签名账户维护 Nonce，而不是按 vault 或 subaccount 维护。因此：

- 一个 `NonceManager` 只属于一个 master account 或 API Wallet；
- 同一 signer 代表多个 vault/subaccount 交易时必须共用该实例；
- 不同 signer 之间禁止共用实例；
- App 多 isolate、多进程或多设备同时使用同一 signer 时，宿主还需要用持久化存储或
  服务端协调高水位；Rust 进程内原子计数不能替代跨进程锁；
- 每次成功分配后应把 `lastIssued` 持久化，重启时恢复；发现外部更高值时调用
  `fast_forward`；
- `next(nowMs)` 原子返回 `max(nowMs, lastIssued + 1)`，即使系统时钟短暂回拨也不会
  重复；网页端的所有 `u64` 参数和返回值都使用十进制字符串；
- `validate_exchange_nonce` 只做官方时间窗口的本地预检，服务器时间与服务端状态仍是
  最终权威。

移动端通过 `FfiNonceManager` 持有 signer 级实例；网页端通过 `WasmNonceManager`
持有实例。不要在 Dart/TypeScript 包中再实现一套独立的自增算法，否则并发规则容易
发生漂移。

## 日志系统

日志默认关闭，由 App 或 Web 在运行时主动开启。日志保存在 Rust 内存中的固定容量
环形缓冲区，不直接写文件，也不会无限增长。缓冲区写满后会丢弃最旧记录，并在
返回结果的 `dropped` 字段中给出丢弃数量。

支持 `error`、`warn`、`info`、`debug`、`trace` 五个级别。每条记录包含：

- `sequence`：进程内递增序列号；
- `timestampMs`：Unix 毫秒时间戳；
- `level`：日志级别；
- `target`：稳定的模块标识；
- `message`：经过长度限制的诊断信息。

移动端通过 `hyper-axis-ffi` 使用以下接口：

- `configure_logging`：开启或关闭日志，并设置级别和容量；
- `snapshot_logs_json`：读取日志但不清空；
- `drain_logs_json`：读取并清空日志；
- `clear_logs`：直接清空日志和溢出计数。

网页端通过 `hyper-axis-wasm` 使用同名导出函数。建议开发环境使用 `debug`，生产
环境默认关闭；需要用户协助排查问题时临时开启 `info`，导出后立即关闭并清空。

内部日志禁止记录私钥、助记词、签名、认证令牌、完整地址、Nonce、价格、数量和
完整请求/响应内容。业务接入层扩展日志时也必须遵守这一安全边界。

## HTTP 与 WebSocket 传输边界

`hyper-axis-transport` 只描述和校验协议，不绑定 Tokio、Dio、Fetch 或浏览器
`WebSocket`。这样移动端和网页端可以使用各自成熟的连接池、代理、证书策略和生命周期
管理，同时共享完全一致的路由、消息格式和错误判定。

HTTP 接入流程：

1. Info 查询优先调用 `build_typed_info_http_request_json`；兼容已有动态查询时可调用
   `build_info_http_request_json`。签名动作最终化后调用 `build_exchange_http_request_json`；
2. 宿主读取 `method`、`url`、`headers`，并把嵌套的 `body` 字符串作为原始请求体发送，
   禁止先在 JavaScript 中解析后再序列化；
3. 收到响应后调用 `parse_http_response_json`，统一取得 `kind`、`retry`、`payload` 和
   `message`；
4. `info` 的 429、5xx、超时或连接失败可退避重试；签名动作遇到同类失败时返回
   `reconcileBeforeRetry`，表示请求可能已经执行，必须先查询订单/账户状态，禁止原样
   自动重放；明确的 API 拒绝返回 `never`。

WebSocket 接入流程：

- 根据 `websocket_url` 连接主网或测试网；
- 使用 `websocket_subscribe_json` / `websocket_unsubscribe_json` 构造完全匹配的订阅；
- 使用 `websocket_typed_info_post_json` 发送经过同一套校验的 Info post；
- 每 50 秒发送 `websocket_ping_json`，服务端 60 秒未收到消息会断开；
- 使用 `parse_websocket_message_json` 区分 `pong`、`subscriptionResponse`、`post` 和
  普通订阅数据，快照消息通过 `isSnapshot` 标记识别；
- 断线后由宿主指数退避重连，并重放仍然有效的订阅；不要重放未确认的签名 `post`，
  应先执行状态对账；
- 单 IP 限制为最多 10 个连接、每分钟 30 个新连接、1000 个订阅、10 个订阅用户、
  每分钟 2000 条发送消息和 100 个在途 `post`。宿主必须在发送队列入口执行限流；
- `fastAssetCtxs` 的数据是 Base64 + raw DEFLATE，解压属于宿主能力，本协议层只保留
  原始 payload，避免强绑定平台压缩库。

## 本地验证

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo clippy -p hyper-axis-wasm --target wasm32-unknown-unknown -- -D warnings
```

## 跨端构建与发布

仓库提供可复现构建脚本：

```bash
scripts/check.sh
scripts/build-linux.sh
scripts/build-android.sh
scripts/build-apple.sh
scripts/build-wasm.sh
```

Linux 产物包含动态库和静态库；Android 产物按 `arm64-v8a`、`armeabi-v7a`、`x86_64`
拆分；Apple 产物分别提供 device 与通用 simulator 静态库；WASM 产物包含 ES module、
`.wasm` 和 TypeScript 声明。CI 对每个提交执行格式、测试、原生/wasm32 严格 Clippy 和
四端制品构建。推送 `v*` tag 后，Release workflow 会复用相同验证流程，生成确定性压缩包
与 `SHA256SUMS` 后发布。

## 当前阶段

当前已完成限价单、触发单、原子批量下单、OID/Cloid 撤单、批量改单、计划撤单、
原生 TWAP、杠杆与隔离保证金管理、子账户创建与资金划转、Vault USD 存取、Core 转账与
桥接提现、API Wallet/Builder Fee 授权、Spot/Perp USD 划转、通用资产转账、Staking
存取和验证者委托、账户与 Agent 抽象模式、多签用户转换、推荐码、EVM 大区块开关、
空 L1 动作、签名节点 jail/unjail、验证者生命周期、gossip priority bid、Spot/Perp
部署器完整动作集、49 类
MultiSig inner action 编排、精度校验、Action 编码、
Action Hash、L1/User-Signed EIP-712 待签摘要、签名恢复
验证、最终请求生成、Nonce 协调、结构化诊断日志、HTTP/WebSocket 协议传输抽象以及
FFI/WASM 边界、官方 38 个 Info 查询方法（37 种 wire type）的强类型请求，以及
Linux/Android/Apple/WASM 的自动化
构建发布流程。当前规划内的 Rust Core、Dart FFI 边界和 Web WASM 边界已全部完成；宿主
仓库只需实现平台网络 I/O、私钥保管/签名以及 UI 状态编排。
