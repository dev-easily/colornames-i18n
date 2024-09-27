### 安装

开发环境不要使用 npm，无法设置这些参数。
```
pnpm config set sharp_binary_host "https://npmmirror.com/mirrors/sharp"
pnpm config set sharp_libvips_binary_host "https://npmmirror.com/mirrors/sharp-libvips"
$ pnpm i
```

### 本地启动

```
$ pnpm start
$ pnpm run start -- --locale en
$ pnpm run start --locale en # macos
```

This command starts a local development server and opens up a browser window. Most changes are reflected live without having to restart the server.

### 编写文档
1. 在 docs 中编写文档
2. 在 blog 中编写新闻
3. 在 src/pages 中增加其它自定义页面
4. 执行 scripts/translate.sh 增加英文
5. 提交

#### 静态资源引用
使用静态资源的法则：
1. 在react页面中，假装自己永远有一个 baseUrl；
2. 在markdown中，假装自己的baseUrl永远是 `/`

### 构建

```
$ pnpm build
```

This command generates static content into the `build` directory and can be served using any static contents hosting service.

### 部署
使用 SSH

```
$ USE_SSH=true pnpm deploy
```

不使用 SSH

```
$ GIT_USER=<Your GitHub username> pnpm deploy
```

推送到 gh-page 分支（已有 github action 任务，无需手动发布）

```
USE_SSH=true GIT_USER=travisbikkle GIT_PASS='秘钥' pnpm deploy
```
