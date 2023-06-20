# erc20
1. 编写erc20的数据结构
<img width="551" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/23b6bc32-db5e-4c8d-99c0-083534ef1b18">

2. 编写erc20数据结构的实现及构造函数
<img width="709" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/b9bd8fde-bfe2-423e-a5cf-2b2a58ebcf17">

3. 编写erc20数据结构总额设置以及账户余额查询方法
<img width="553" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/78bb2b66-c244-4ac0-98da-ba0747be901b">

4. 编写自身转账及操作第三方账户转账方法
<img width="716" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/6bc79fef-9d19-4f3f-95ec-22d2e8ab1e4c">
<img width="729" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/bc853e47-8e00-4831-bdd5-7fda1845057c">

5. 编写授权第三方账户操作给定余额的方法
<img width="638" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/9d6371bc-9b44-41c8-b932-0f00fd2649d9">

6. 编译合约，启动Contract-node，打开[contract-ui.substrate.io](https://contracts-ui.substrate.io/),上传编译后的erc20.contract文件，调用new方法初始化（Alice账号）总额100000
<img width="1006" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/c52d7884-0bb2-4dfa-ba40-1ea3d0d70eb7">
<img width="1011" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/99dd6080-f309-4e2a-8679-7ad67be3e257">

7. 查询Alice 余额
<img width="985" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/dff025b8-8811-480a-9151-2e8461516772">

8. Alice给Bob 转账 10000
<img width="1010" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/97a6504d-97de-4596-afd4-27e48f875e74">
<img width="970" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/bfca6609-7368-4757-9a75-83f24528c895">


9. 未授权前，Bob操作Alice账号 给 Alice_stash 转账 100 (报错：AllowanceTooLow，即授权额度不足)
<img width="983" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/ff7e854a-ddd3-42b9-8557-d828df2f2fd3">

10. Alice向Bob授权10000
<img width="1013" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/9ff71aa7-669a-43de-8d1a-65279576f18c">

11. 授权成功后，Bob操作Alice账号 给Alice_stash 转账 100（成功）
<img width="1014" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/ddd17ed2-c010-46ca-8727-b24bf4e15987">
<img width="982" alt="image" src="https://github.com/TerryTyh/erc20/assets/120092281/979b74d9-9a41-4525-b189-238274c319d9">



