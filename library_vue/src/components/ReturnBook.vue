<template>
    <div>
        <el-table
            :data="refreshTableData.slice((currentPage-1)*pageSize,currentPage*pageSize)"
            style="width: 100%;margin-top:20px" border :height="580"
            stripe empty-text="暂无数据" >
            <el-table-column  prop="id" label="借阅编号" min-width="1" sortable align="center"></el-table-column>
            <el-table-column prop="book_id" label="书籍编号" min-width="1" align="center"></el-table-column>
            <el-table-column prop="borrow_time" label="借阅时间" min-width="1" align="center" ></el-table-column>
            <el-table-column prop="return_time" label="归还时间" min-width="1" align="center"></el-table-column>
            <el-table-column label="状态" min-width="1" align="center" :formatter="getReturn" column-key="status" ></el-table-column>
            <el-table-column   min-width="1" align="center">
                <template #default="scope">
                    <el-button link type="primary" size="large" @click="returnb(scope.row)">
                        归还
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
    name: "ReturnBook",
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
        this.getAllRecords();
    },
    computed:
    {
        refreshTableData()
        {
            var temData = this.allData;
            temData = temData.filter(data=> !this.search || data.name.toLowerCase().includes(this.search.toLowerCase()))
            temData.forEach((item,index)=>{
              item.borrow_time = item.borrow_time.substr(0,10);  
              if(item.return_time!=null)
              {
                item.return_time = item.return_time.substr(0,10);  
              }
              
            })
            this.currentPage = 1;
            return temData;
        }
    },
    methods:
    {
        getReturn(row)
        {
            return row.is_return?'已归还':'未归还';
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
        getAllRecords()
        {
            console.log(this.user)
            this.$axios({
              methods:'GET',
              url:this.$rust + '/users/'+ this.user.id + '/return/',
            })
            .then(res=>{
                console.log('返回数据成功',res.data);
                this.allData = res.data;

            })
            .catch(res=>{
            console.log('返回数据失败',res);
          })
        },

        returnb(row)
        {
            if(row.is_return==true)
            {
                this.$message.error("该书籍已归还");
            }
            console.log("id",row);
            this.$axios.post(
                this.$rust + '/users/'+ this.user.id + '/return/',{
                    record_id: row.id,
                    book_id:row.book_id
                }
            )
            .then(res=>{
                this.$message.success("归还成功");
                this.getAllRecords();
                return;
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
