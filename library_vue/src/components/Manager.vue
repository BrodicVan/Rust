<template>
    <div class="whole">
        <el-container>
            <el-aside class="aside" >
                <h1>管理员菜单</h1>
                <el-col class="tac" >
                    <el-menu
                        default-active="1"
                        background-color='#4edea4'
                        text-color="#fff"
                        active-text-color="#000000"
                        
                    >
                        <el-menu-item index="1" @click="changeChoice(1)">
                            <div class="menu_font">
                                <p>查看用户</p>
                            </div>
                            
                        </el-menu-item>
                        <el-menu-item index="2" @click="changeChoice(2)">
                            <div class="menu_font">
                                <p>借书记录</p>
                            </div>
                        </el-menu-item>
                        <el-menu-item index="3" @click="changeChoice(3)">
                            <div class="menu_font">
                                <p>图书管理</p>
                            </div>
                        </el-menu-item>
                    </el-menu>
                </el-col>
            </el-aside>
            <el-main >
                <AllUserInfo v-if="choice==1"></AllUserInfo>
                <AllRecord v-if="choice==2"></AllRecord>
                <ManageBook v-if="choice==3"></ManageBook>
            </el-main>
        </el-container>
        
    </div>
    
</template>



<script>
import AllUserInfo from './AllUserInfo.vue';
import AllRecord from './AllRecord.vue'
import ManageBook from './ManageBook.vue';

export default 
{
    name: "Manager",
    data() {
        return {
            choice: 1
        };
    },
    created() {
    },
    mounted() {
        let t = JSON.parse(sessionStorage.getItem("user")||"{}")
        if(t==null || t=={})
        {
            this.$router.push('/login')
            return
        }
        if( !t.is_mana)
        {
            this.$message.error("权限不足");
            this.$router.push('/reader')
        }
    },
    computed: {},
    methods: {
        changeChoice(c)
        {
            this.choice = c;
        }
    },
    components: { AllUserInfo, AllRecord, ManageBook }
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
        text-align: center;
   }

   .aside
   {
        background-color: #4edea4;
        text-align: center;
   }

   .el-container {
    height: 100%;
  }

  .el-aside {
    background-color: #7ca9dd;
    color: #333;
    line-height: 200px;
  }

  .el-main {
    background-color: #9ea4aa;
  }

  .menu_font
  {
    font-weight: 500;
    font-size: xx-large;
    text-align: center;
  }
</style>
