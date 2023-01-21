<template>
    <div class="my_whole" >
        <span>
            <div class="login_area">
                <div style="font-size: 30px; text-align: center; padding: 30px 0; color: #333">欢迎登录</div>
                <el-form ref="form" :rules="rules" :model="form">
                    <el-form-item prop="id">
                        <el-input  v-model="form.id" placeholder="请输入用户ID..."></el-input>
                    </el-form-item>
                    
                    <el-form-item prop="pwd">
                        <el-input  v-model="form.pwd" show-password
                                placeholder="请输入密码..."></el-input>
                    </el-form-item>
                    
                    <el-form-item>
                        <el-button style="width: 100%;background-color: #00b385;:" type="primary" @click="login">登 录</el-button>
                    </el-form-item>
                </el-form>
                <el-row>
                    <el-button type="text" @click="$router.push('/reg')" style="color: #00b385;:">前往注册 >></el-button>
                </el-row>
                
            </div>
        </span>
        
        
        
    </div>
    
</template>



<script>


export default 
{
    name: "Login",
    data () 
    {
        return {
            form:
            {
                id:'',
                pwd:''
            },
            rules:
            {
                id: 
                [
                    { required: true, message: '请输入用户ID', trigger: 'blur' },
                    { type: 'number', message: '请输入纯数字', trigger: 'change' },
                    { min: 1, max: 10, message: '长度在 1 到 10 个字符', trigger: 'blur' },
                    
                ],
                pwd:
                [
                    { required: true, message: '请输入用户密码', trigger: 'blur' },
                    { min: 5, max: 10, message: '长度在 5 到 10 个字符', trigger: 'blur' },
                    
                ],
            }
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
            let t = JSON.stringify({
                id:1,
                password:'123'
            })
            console.log("ttt",this.t);
            this.$axios.post('http://127.0.0.1:3333/login/',{
                id:1,
                password:'13'
            }
            ).then(res=>
                {
                    console.log(res);
                }
            ).catch(res=>
                {
                    console.log(res);
                }
            )
        }
    }
}
</script>
    
<style scoped>
    .my_whole
    {
        width: 100%;
        height: 100%;
        /* display: flex;
        justify-content: center;
        align-items: center; */
    }
    .login_area
    {
        width: 400px;
        display: inline;
        vertical-align: middle;
    }

    span
    {
        display: inline;
        vertical-align: middle;
    }
    
</style>
