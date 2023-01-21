<template>
    <div  class="login_area">
            <div style="font-size: 30px; text-align: center; padding: 30px 0; color: #333">欢迎注册</div>
            <el-form ref="form" :rules="rules" :model="form">
                <el-form-item prop="id">
                    <el-input  v-model="form.id" placeholder="请输入用户名" oninput="value=value.replace(/[^\d]/g,'')"></el-input>
                </el-form-item>
                
                <el-form-item prop="pwd">
                    <el-input  v-model="form.pwd" show-password
                            placeholder="请输入密码..."></el-input>
                </el-form-item>
                
                <el-form-item prop="again_pwd">
                    <el-input  v-model="form.pwd" show-password
                            placeholder="请再次输入密码..."></el-input>
                </el-form-item>
                
                <el-form-item>
                    <el-button style="width: 100%;background-color: #00b385;:" type="primary" @click="login">注册</el-button>
                </el-form-item>
            </el-form>
            <div style="text-align:left">
                <el-button text @click="$router.push('/login')" style="color: #00b385;:">&lt;&lt;返回登录</el-button>
            </div>
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
                pwd:'',
                again_pwd:''
            },
            rules:
            {
                id: 
                [
                    { required: true, message: '请输入用户ID', trigger: 'blur' },
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
    mounted()
    {
        sessionStorage.removeItem("user");
    },
    computed:
    {
        
    },
    methods:
    {
        login()
        {
            this.$refs.form.validate((valid) => {
                if (valid) 
                {
                    let f = 
                    {
                        id: parseInt(this.form.id),
                        password: this.form.pwd
                    }
                    alert('submit!');
                    this.$axios.post('http://127.0.0.1:3333/login/',f
                    ).then(res=>
                        {
                            console.log(res);
                            sessionStorage.setItem("user", JSON.stringify(res.data))
                        }
                    ).catch(res=>
                        {
                            console.log(res);
                        }
                    )
                } 
                else 
                {
                    this.$message.error("请先正确填写信息");
                    console.log('error submit!!');
                }
        });
            
        }
    }
}
</script>
    
<style>
    .reg_area
    {
        
        width: 30vw; 
        height: 30vh;
        margin:  35vh 35vw 35vh 35vw;
        background-color: #65dfe5;
        border-width: 5px;
        border-color: black;
        border-radius: 5px;
    }
</style>
