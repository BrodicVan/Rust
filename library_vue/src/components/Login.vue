<template>
    <div  class="login_area">
            <div style="font-size: 30px; text-align: center; padding: 30px 0; color: #333">欢迎登录</div>
            <el-form ref="form" :rules="rules" :model="form">
                <el-form-item prop="id">
                    <el-input  v-model="form.id" placeholder="请输入纯数字用户ID..." oninput="value=value.replace(/[^\d]/g,'')"></el-input>
                </el-form-item>
                
                <el-form-item prop="pwd">
                    <el-input  v-model="form.pwd" show-password
                            placeholder="请输入密码..."></el-input>
                </el-form-item>
                
                <el-form-item>
                    <el-button style="width: 100%;background-color: #00b385;:" type="primary" @click="login">登 录</el-button>
                </el-form-item>
            </el-form>
            <div style="text-align:right">
                <el-button text @click="$router.push('/reg')" style="color: #00b385;:">前往注册 >></el-button>
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
                pwd:''
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
                    this.$axios.post(this.$rust + '/login/',f
                    ).then(res=>
                        {
                            sessionStorage.setItem("user", JSON.stringify(res.data))
                            let t = JSON.parse(sessionStorage.getItem("user")||"{}");
                            this.form.pwd='';
                            if(t.is_mana)
                            {
                                this.$router.push('/manager');
                                
                            }
                            else
                            {
                                this.$router.push('/reader');
                            }
                        }
                    ).catch(res=>
                        {
                            console.log(res.response)
                            this.$message.error(res.response.data.error_message)
                        }
                    )
                } 
                else 
                {
                    sessionStorage.removeItem("user");
                    this.$message.error("请先正确填写信息");
                    console.log('error submit!!');
                }
        });
            
        }
    }
}
</script>
    
<style>
    .login_area
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
