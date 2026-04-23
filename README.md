快速开始

1. 创建Cloudflare资源
    ```bash
    npx wrangler d1 create demo_user_d1 
    npx wrangler queues create demo-user-queue
    ```
2. 初始化数据库
    ```bash
    npx wrangler d1 execute demo_user_d1 --local --file ./migrations/init.sql
    npx wrangler d1 execute demo_user_d1 --remote --file ./migrations/init.sql
    ```
3. 本地调试或部署
    ```bash
    npx wrangler dev
    npx wrangler deploy
    ```