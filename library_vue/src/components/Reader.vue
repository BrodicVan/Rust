<template>
    <div class="whole">
        <el-container>
            <el-aside class="aside" >
                <h1>读者菜单</h1>
                <el-col class="tac" >
                    <el-menu
                        default-active="1"
                        background-color='#4edea4'
                        text-color="#fff"
                        active-text-color="#000000"
                        
                    >
                        <el-menu-item index="1" @click="changeChoice(1)">
                            <div class="menu_font">
                                <p>图书借阅</p>
                            </div>
                            
                        </el-menu-item>
                        <el-menu-item index="2" @click="changeChoice(2)">
                            <div class="menu_font">
                                <p>图书归还</p>
                            </div>
                        </el-menu-item>
                    </el-menu>
                </el-col>
            </el-aside>
            <el-main >
                <BorrowBook v-if="choice==1"></BorrowBook>
                <ReturnBook v-if="choice==2"></ReturnBook>
            </el-main>
        </el-container>
        
    </div>
    
</template>



<script>
import BorrowBook from './BorrowBook.vue';
import ReturnBook from './ReturnBook.vue';



export default 
{
    name: "Reader",
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
        
        if( t.is_mana)
        {
            this.$message.error("权限不足");
            this.$router.push('/manager')
        }
    },
    computed: {},
    methods: {
        changeChoice(c)
        {
            this.choice = c;
        }
    },
    components: { BorrowBook, ReturnBook }
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
