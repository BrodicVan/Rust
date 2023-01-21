# Rust大作业运行指南

## 后端部署

端口号选择了本地端口号3333  

1.数据库建立  
使用postgreSql数据库执行library_rust下的sql文件  
2.DATABASE_URL设置  
修改library_rust下的.env文件中的DATABASE_URL  
DATABASE_URL=postgres://{username}:{password}@localhost:{port}/{database_name}
{}内的内容需要自行填入,username默认postgres，port默认5432  
3.运行指令  
到library_rust/webservice目录下运行指令`cargo run`即可  

