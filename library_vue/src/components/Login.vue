<template>
    <div class="whole">
        <div class="container">
            <div class="login-wrapper">
                <div>登&emsp;录</div>
                <div class="form-wrapper">
                    <el-input placeholder="用户编号"> </el-input>
                    <el-input placeholder="用户密码"> </el-input>
                    <el-button type="primary" @click="login">登 录</el-button>
                </div>
                <div>
                    没有账号?
                    <a href="#" class="reg">注 册</a>
                </div>
            </div>
        </div>
    </div>
    
</template>



<script>
import axios from 'axios'
export default 
{
    name: "Login",
    data () 
    {
        return {
            
        }
    },
    created()
    {
        
    },
    computed:
    {
        
    },
    methods:
    {
        login()
        {
            axios.get("/123").then((response) => {console.log(response.data)})
        }
    }
}
</script>
    
<style scoped>
    .whole
    {
        text-align: center;
    }
    .container
    {
        text-align: center;
        width: 50%;
    }
    
    .login_wrapper
    {
        width: 50%;
    }
</style>
