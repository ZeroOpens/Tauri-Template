# Tauri2.0 模版项目

<p align="center"><a href="https://tauri.app/start/"><img width="100" src="./src/assets/Images/MKImages/logo.png" alt="logo"></a></p>

<h3 align="center">使用 Vue3 + Ts 的 Tauri2.0 模版</h3>
<br>

<p align="center">
  <a href="https://cn.vuejs.org/guide/introduction.html">
    <img alt="Vue3" src="https://img.shields.io/badge/%E6%A1%86%E6%9E%B6-Vue3-%2341B883?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI1MDAiIGhlaWdodD0iNDMxLjY0MDYyNDk5OTk5OTk0IiB2aWV3Qm94PSIwIDAgMjU2IDIyMSI%2BPHBhdGggZmlsbD0iIzQxQjg4MyIgZD0iTTIwNC44IDBIMjU2TDEyOCAyMjAuOEwwIDBoOTcuOTJMMTI4IDUxLjJMMTU3LjQ0IDB6Ii8%2BPHBhdGggZmlsbD0iIzQxQjg4MyIgZD0ibTAgMGwxMjggMjIwLjhMMjU2IDBoLTUxLjJMMTI4IDEzMi40OEw1MC41NiAweiIvPjxwYXRoIGZpbGw9IiMzNTQ5NUUiIGQ9Ik01MC41NiAwTDEyOCAxMzMuMTJMMjA0LjggMGgtNDcuMzZMMTI4IDUxLjJMOTcuOTIgMHoiLz48L3N2Zz4%3D
">
    <a href="https://www.sass.hk/guide/">
    <img alt="Scss" src="https://img.shields.io/badge/%E8%AF%AD%E8%A8%80-Scss-%23CC6699?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI1MDAiIGhlaWdodD0iNDQ1LjMxMjUiIHZpZXdCb3g9IjAgMCAyNTYgMjI4Ij48cGF0aCBmaWxsPSIjRkYwMDgwIiBkPSJNMCAxMTAuNTU4di0zLjIyYzEyLjU0Ni0xMC42NDUgMjguMDQzLTI1LjUgMzkuNjA1LTM4LjEyN2w1LjkwMyAzLjk2MWMtNi42NCAxMS44ODUtMTYuMjM0IDI0Ljc1OS0yNS41ODIgMzQuNDE1YzE0Ljc2IDE4LjU2OSAxNS40OTcgMTkuNTYgMjUuMDkgMzUuNDA2bC01LjQxMSAzLjk2MmMtNS40MTItNS42OTQtNy44NzItOC40Mi0xNS40OTgtMTUuMzVjLTguODU1LTguNDItNy44NzItNy4xODItMjAuNjYzLTE4LjA3NnptNTAuMjA0IDU3LjU1MmwxMC41MS01LjI1NGMxOS43MDUgMjkuODggNTAuOTA1IDUwLjU2NiA3NS44NjUgNTAuNTY2YzIzLjk3NSAwIDQxLjA1NC0xNy4wNzUgNDEuMDU0LTQxLjM3MmMwLTIzLjk2OC0xMy43OTUtMzYuMTE3LTUzLjUzMy00Ni45NTJjLTQ3LjI5My0xMi44MDYtNjUuMzU2LTMwLjIwOC02NS4zNTYtNjQuMDI2QzU4Ljc0NCAyNy4yNTIgODUuMDE3IDAgMTE3LjIwMyAwYzguODY3IDAgOC4yMSAwIDI4LjkwMSA2LjIzOWMyLjk1NS45ODUgNi44OTcgMS42NDIgOC41MzkgMS42NDJjMi4yOTggMCA1LjU4My0uOTg1IDguNTQtMi4yOTlMMTc0LjM0Ni4wMDFsMjAuMzYzIDU2LjgwM2wtMTAuNTEgNi44OTVjLTE5LjA0OC0zMS41MjEtNDMuMzUyLTQ5LjU4LTY2Ljk5Ny00OS41OGMtMjAuNjkgMC0zNi4xMjYgMTUuNDMyLTM2LjEyNiAzNi40NDZjMCAyMC42ODUgMTQuMTIyIDM0LjE0OCA0My4zNTEgNDEuN2MxNC4xMjMgMy42MTIgMjQuMzAzIDYuNTY3IDI5LjIzIDcuODhjMjkuNTU4IDguODY0IDQ3LjI5MyAzMi4xNzggNDcuMjkzIDYxLjRjMCAzNy4xMDMtMjguNTcyIDY1Ljk5Ny02NC42OTkgNjUuOTk3Yy0xMS4xNjYgMC0xNi43NS0xLjMxMy0zMC41NDMtNy41NTJjLTUuOTExLTIuNjI3LTkuNTI0LTMuNjExLTExLjgyMy0zLjYxMWMtMy4yODQgMC01LjU4My45ODUtOS41MjQgMy4yODNsLTEyLjQ4IDcuODh6bTE2MS4wMjUtMjUuMTE2YzkuNTk0LTE2LjA5NCAxMC4wODYtMTYuODM2IDI0Ljg0Ni0zNS40MDZjLTkuMzQ4LTkuNjU3LTE4Ljk0Mi0yMi41My0yNS41ODMtMzQuNDE2bDYuMTQ4LTMuOTYxYzExLjMxNyAxMi42MjcgMjYuMzIyIDI2Ljk4NyAzOS4zNiAzOC4xMjh2My4yMmwtMTIuNTQ2IDEwLjM5OGMtNC42NzQgMy45NjItMTguMjAzIDE2LjgzNy0yNy4wNTkgMjUuOTk4eiIvPjwvc3ZnPg%3D%3D
">
  </a>
    <a href="https://www.tslang.cn/">
    <img alt="TypeScript" src="https://img.shields.io/badge/%E8%AF%AD%E8%A8%80-TypeScript-%230E80C1?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI1MDAiIGhlaWdodD0iNTAwIiB2aWV3Qm94PSIwIDAgMTYgMTYiPjxwYXRoIGZpbGw9IiMwRTgwQzEiIGQ9Ik0xMyA5Ljc1QTIuMjggMi4yOCAwIDAgMSAxMC41IDEyQTIuMjggMi4yOCAwIDAgMSA4IDkuNzVhLjUuNSAwIDAgMSAxIDBjMCAuOTI0LjgwOCAxLjI1IDEuNSAxLjI1czEuNS0uMzI2IDEuNS0xLjI1YzAtLjYwOC0uNjMzLS44OS0xLjY3Ni0xLjI4MVM4IDcuNiA4IDYuMjVBMi4yOCAyLjI4IDAgMCAxIDEwLjUgNEEyLjI4IDIuMjggMCAwIDEgMTMgNi4yNWEuNS41IDAgMCAxLTEgMEMxMiA1LjMyOCAxMS4xOTIgNSAxMC41IDVTOSA1LjMyOCA5IDYuMjVjMCAuNjA5LjYzMy44OSAxLjY3NiAxLjI4MkMxMS43MTkgNy45MjMgMTMgOC40IDEzIDkuNzVNNi43NSA0aC00LjVhLjUuNSAwIDEgMCAwIDFINHY2LjVhLjUuNSAwIDEgMCAxIDBWNWgxLjc1YS41LjUgMCAxIDAgMC0xIi8%2BPC9zdmc%2B
">
  </a>
</p>

---

### <img width="20" src="./src/assets/Images/MKImages/项目介绍.png" style="vertical-align: middle;" alt="项目介绍"> 项目介绍

本项目为 Tauri2.0 模版项目，拉取后直接可以编写，无需额外的配置。

### <img width="19" src="./src/assets/Images/MKImages/添加功能.png" style="vertical-align: middle;" alt="模版已添加功能"> 模版已添加功能

1、添加axios，已配置拦截器<br>
2、添加router，已配置示例<br>
3、添加pinia，已配置持久化<br>
4、添加更新插件，修改私钥和请求地址即可<br>

### <img width="19" src="./src/assets/Images/MKImages/插件.png" style="vertical-align: middle;" alt="需要安装的插件"> 需要安装的插件

1. <img width="50" src="./src/assets/Images/MKImages/Vue - Official.png" alt="Vue - Official"> Vue - Official

2. <img width="50" src="./src/assets/Images/MKImages/logo.png" alt="Tauri"> Tauri


### <img width="20" src="./src/assets/Images/MKImages/使用教程.png" style="vertical-align: middle;" alt="使用教程"> 使用教程

<a href="https://tauri.app/start/prerequisites/">点击查看前置条件</a>

1. 拉取项目

```shell
# git
git clone https://gitee.com/ZeroOpens/tauri-template.git
```

2. 安装依赖

```shell
# npm
npm install

# pnpm
pnpm install
```

3. 运行程序

```shell
# 运行
pnpm tauri dev
```
4. 打包程序

```shell
# 打包
pnpm tauri build
```

### <img width="20" src="./src/assets/Images/MKImages/QQ.png" style="vertical-align: middle;" alt="QQ交流群"> QQ 交流群

QQ 群号：863411527
<br>
扫码添加:
<br>
<img width="150" src="./src/assets/Images/MKImages/QQ交流群.png" alt="QQ">

### <img width="20" src="./src/assets/Images/MKImages/工程结构解析.png" style="vertical-align: middle;" alt="工程主要结构解析"> 工程主要结构解析

```
Tauri-Template
├── Server                     # 服务器代码
│   └── update                 # 更新功能代码
├── src                        # 前端源代码
│   ├── components             # 全局组件
│       └── update.vue         # 更新组件
│   ├── hooks                  # 组合式函数  
│       └── useTauri.ts        # 获取 Tauri 信息
│   ├── pages                  # 主包页面
│       └── index.vue          # 首页
│   ├── router                 # 路由
│       └──index.ts            # 路由配置
│   ├── services               # 所有请求
│       └── index.ts           # 请求示例
│   ├── assets                 # 本地静态资源
│       └── images             # 图片
│   ├── stores                 # 全局 pinia
│       ├── modules            # pinia 仓库
│       └── index.ts           # 全局 pinia 配置
│   ├── types                  # 类型声明文件
│       └── index.d.ts         # 全局组件类型声明
│   ├── utils                  # 全局方法
│       └── http.ts            # axios 拦截器
│   ├── App.vue                # 入口页面
│   └── main.ts                # Vue初始化入口文件
├── src-tauri                  # Rust 后端源代码
│   ├── icons                  # 应用图标
│   ├── src                    # Rust 源文件目录
│       └── main.rs            #  Rust 程序的入口
│   ├── tauri.conf.json        # Tauri 项目的核心配置
│   └── Cargo.toml             # Rust 项目的依赖
├── .gitignore                 # git 忽略文件
├── index.html                 # H5 端首页
├── LICENSE                    # 开源协议
├── package.json               # package.json 依赖
├── README.md                  # Markdown 说明文档
└── vite.config.ts             # vite 配置
```

### <img width="20" src="./src/assets/Images/MKImages/版本更新.png" style="vertical-align: middle;" alt="版本更新内容"> 版本更新内容

#### v1.2.1 (2025-05-10)

1.  添加router。
2.  添加pinia、持久化插件。
3.  添加更多的思路。
4.  添加更多示例。

#### v1.1.1 (2025-05-06)

1.  添加axios和拦截器。
2.  添加跨域问题解决方法。
3.  添加更多示例。

#### v1.1.0 (2025-05-06)

1.  添加更多示例。
2.  修改、删除多余文件。
3.  配置 @ 指定前端src为顶级文件。

#### v1.0.0 (2025-05-05)

1.  新版本发布。
