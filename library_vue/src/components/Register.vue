<template>
    <div  class="login_area">
            <div style="font-size: 30px; text-align: center; padding: 30px 0; color: #333">欢迎注册</div>
            <el-form ref="form" :rules="rules" :model="form">
                <el-form-item prop="name">
                    <el-input  v-model="form.name" placeholder="请输入用户名" ></el-input>
                </el-form-item>
                
                <el-form-item prop="pwd">
                    <el-input  v-model="form.pwd" show-password
                            placeholder="请输入密码..."></el-input>
                </el-form-item>
                
                <el-form-item prop="again_pwd">
                    <el-input  v-model="form.again_pwd" show-password
                            placeholder="请再次输入密码..."></el-input>
                </el-form-item>
                
                <el-form-item>
                    <el-button style="width: 100%;background-color: #00b385;:" type="primary" @click="reg">注册</el-button>
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
    name: "reg",
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
                name: 
                [
                    { required: true, message: '请输入用户名', trigger: 'blur' },
                    { min: 1, max: 10, message: '长度在 1 到 10 个字符', trigger: 'blur' },
                    
                ],
                pwd:
                [
                    { required: true, message: '请输入用户密码', trigger: 'blur' },
                    { min: 5, max: 10, message: '长度在 5 到 10 个字符', trigger: 'blur' },
                    
                ],
                again_pwd:
                [
                    { required: true, message: '请再次输入用户密码', trigger: 'blur' },
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
        reg()
        {
            this.$refs.form.validate((valid) => {
                if (valid) 
                {
                    if(this.form.pwd==this.form.again_pwd)
                    {
                        let f = 
                        {
                            name: this.form.name,
                            password: this.form.pwd
                        }
                        alert('submit!');
                        this.$axios.post('http://127.0.0.1:3333/reg/',f
                        ).then(res=>
                            {
                                this.$message.success('注册成功，请牢记你的id：'+res.data.id);
                                console.log(res);
                                sessionStorage.setItem("user", JSON.stringify(res.data))
                                this.$router.push('/reader');
                            }
                        ).catch(res=>
                            {
                                console.log(res);
                            }
                        )
                    }
                    else{
                        this.$message.error("两次密码不一致！");
                        console.log('error submit!!');
                    }
                    
                    
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
