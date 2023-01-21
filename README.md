# Rust大作业运行指南

## 后端服务器部署

后端端口号选择了本地端口号3333  

- 数据库建立  
  - 使用postgreSql数据库执行library_rust下的sql文件   
  - DATABASE_URL设置  
修改library_rust下的.env文件中的DATABASE_URL  
`DATABASE_URL=postgres://{username}:{password}@localhost:{port}/{database_name}`
{}内的内容需要自行填入,username默认postgres，port默认5432  
- 运行指令  
在library_rust/webservice目录下运行指令`cargo run`即可（如报错，尝试cargo clean后cargo run）  

## 前端服务器部署

前端端口可自主选择，运行步骤：
+ 安装node.js，并使用npm install -g http-server指令安装http-server
+ 在library_vue/dist目录下运行指令`http-server`即可完成部署，在浏览器中输入对应地址即可访问网站
