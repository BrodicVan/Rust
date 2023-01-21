<template>
    <div>
        <el-input v-model="search" placeholder="输入书名进行搜索..."></el-input>
        <el-table
            :data="refreshTableData.slice((currentPage-1)*pageSize,currentPage*pageSize)"
            style="width: 100%;margin-top:20px" border :height="580"
            :default-sort = "{prop: 'date', order: 'ascending'}" stripe empty-text="暂无数据" >
            <el-table-column  prop="id" label="书籍编号" min-width="1" sortable align="center"></el-table-column>
            <el-table-column prop="name" label="书名" min-width="1" align="center"></el-table-column>
            <el-table-column prop="writer" label="作者" min-width="1" align="center" ></el-table-column>
            <el-table-column prop="press" label="出版社" min-width="1" align="center"></el-table-column>
            <el-table-column label="状态" min-width="1" align="center" :formatter="getBorrowed" column-key="status" ></el-table-column>
            <el-table-column   min-width="1" align="center">
                <template #default="scope">
                    <el-button link type="primary" size="large" @click="borrow(scope.row)">
                        借阅
                    </el-button>
                </template>
            </el-table-column>
        </el-table>
        <el-row justify="center" style="margin-top:20px">
            <el-pagination
                align='center'
                @size-change="handleSizeChange"
                @current-change="handleCurrentChange"
                :current-page="currentPage"
                :page-sizes="[1,5,10,20]"
                :page-size="pageSize"
                layout="total, sizes, prev, pager, next, jumper"
                :total="refreshTableData.length">
            </el-pagination>
        </el-row>
    </div>
</template>



<script>
export default {
    name: "BorrowBook",
    data () {
        return {
            currentPage: 1,
            pageSize: 10,
            search: '',
            allData:[],
            tableData: [],
            user:{}
        }
    },
    created()
    {
        this.user = JSON.parse(sessionStorage.getItem("user")||"{}");
    },
    mounted()
    {
        this.getAllBooks();
    },
    computed:
    {
        refreshTableData()
        {
            var temData = this.allData;
            temData = temData.filter(data=> !this.search || data.name.toLowerCase().includes(this.search.toLowerCase()))
            this.currentPage = 1;
            return temData;
        }
    },
    methods:
    {
        getBorrowed(row)
        {
            return row.is_borrowed?'外借':'在架';
        },
        //每页条数改变时触发 选择一页显示多少行
        handleSizeChange(val)
        {
            console.log(`每页 ${val} 条`);
            this.currentPage = 1;
            this.pageSize = val;
        },
        //当前页改变时触发 跳转其他页
        handleCurrentChange(val)
        {
            console.log(`当前页: ${val}`);
            this.currentPage = val;
        },
        getAllBooks()
        {
            console.log(this.user)
            this.$axios({
              methods:'GET',
              url:this.$rust + '/users/'+ this.user.id + '/borrow/',
            })
            .then(res=>{
                console.log('返回数据成功',res.data);
                this.allData = res.data;

            })
            .catch(res=>{
            console.log('返回数据失败',res);
          })
        },

        borrow(row)
        {
            if(row.is_borrowed==true)
            {
                this.$message.error("该书籍已被借阅");
            }
            console.log("id",row);
            this.$axios.post(
                this.$rust + '/users/'+ this.user.id + '/borrow/',{
                    id: row.id
                }
            )
            .then(res=>{
                this.$message.success("借阅成功");
                this.getAllBooks();
            })
            .catch(res=>{
                this.$message.error(res.response.data.error_message);
          })
        }
    }
}
</script>

<style scoped>




</style>
