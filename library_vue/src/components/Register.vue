<template>
    <div class="my_whole" align="center">
        <div style="width: 400px; margin: 100px auto">
            <div style="font-size: 30px; text-align: center; padding: 30px 0; color: #333">注册</div>
            <el-form ref="form" :model="form" :rules="rules">
                <el-form-item prop="email">
                    <el-input  v-model="id" placeholder="请输入密码..."></el-input>
                </el-form-item>
                <el-form-item prop="password">
                    <el-input  v-model="pwd" show-password
                            placeholder="再次输入密码..."></el-input>
                </el-form-item>
                <el-form-item>
                    <el-button style="width: 100%;background-color: #00b385;:" type="primary">注 册</el-button>
                </el-form-item>
                <el-form-item style="text-align: left;">
                    <el-button type="text" @click="$router.push('/login')" style="color: #00b385;:">&lt;&lt;返回登录</el-button>
                </el-form-item>
            </el-form>
        </div>
        
        
    </div>
    
</template>



<script>
import axios from 'axios'
export default 
{
    name: "Register",
    data () 
    {
        return {
            id: '',
            pwd:''
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
            axios.get("/123").then((res) => {console.log(res.data)}).catch((res)=>{console.log(res.code)})
        }
    }
}
</script>
    
<style scoped>
    .my_whole
    {
        text-align: center;
        width: 100%;
        height: 100%;
        
    }
    .login_area
    {
        width: 50%;
    }
    
</style>
