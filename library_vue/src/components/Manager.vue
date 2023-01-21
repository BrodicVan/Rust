<template>
    <div class="whole">
        <el-container>
            <el-aside class="aside">
                <h1>管理员菜单</h1>
                <el-col class="tac">
                    <el-menu
                        default-active="1"
                        background-color='#4edea4'
                        text-color="#fff"
                        
                        active-text-color="#000000"
                    >
                        <el-menu-item index="1">
                            <span >Navigator Three</span>
                        </el-menu-item>
                        <el-menu-item index="2">
                            <span >Navigator Four</span>
                        </el-menu-item>
                    </el-menu>
                </el-col>
            </el-aside>
            <el-main>
                
            </el-main>
        </el-container>
        
    </div>
    
</template>



<script>
import user_info from './AllUserInfo.vue';

export default 
{
    name: "Manager",
    data () 
    {
        return {
            
        }

    },
    created()
    {
        
    },
    mounted()
    {
        if(JSON.parse(sessionStorage.getItem("user")||"{}")=={})
        {
            this.$router.push('/login');
        }
    },
    computed:
    {
        
    },
    methods:
    {
        
            
        
    }
}
</script>
    
<style scoped>
    .whole
    {
        height: 100%;
    }
   .tac
   {
        background-color: #4edea4;
   }

   .aside
   {
        background-color: #4edea4;
   }

   .el-container {
    height: 100%;
  }

  .el-header, .el-footer {
    background-color: #B3C0D1;
    color: #333;
    text-align: center;
    height: 60px !important;
  }

  .el-aside {
    background-color: #D3DCE6;
    color: #333;
    text-align: center;
    line-height: 200px;
  }

  .el-main {
    background-color: #9ea4aa;
  }
</style>
