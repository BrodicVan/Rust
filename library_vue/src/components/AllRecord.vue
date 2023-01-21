<template>
    <div>
        <el-table 
            :data="refreshTableData.slice((currentPage-1)*pageSize,currentPage*pageSize)"
            style="width: 100%" border
            :default-sort = "{prop: 'id', order: 'ascending'}" stripe height="660">
            <el-table-column min-width="1"  prop="id" label="借阅编号"  sortable align="center"></el-table-column>
            <el-table-column min-width="1" prop="user_id" label="读者编号"  align="center"></el-table-column>
            <el-table-column min-width="1" prop="book_id" label="书籍编号" align="center" ></el-table-column>
            <el-table-column min-width="1" prop="borrow_time" label="借阅时间"  align="center"  ></el-table-column>
            <el-table-column min-width="1" :formatter="getReturnTime" label="归还时间"  align="center"  ></el-table-column>
        </el-table>
        <el-row justify="center" style="margin-top:20px">
            <el-pagination
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


export default 
{
    name: "AllRecord",
    data () {
        return {
            currentPage: 1,
            pageSize: 10,
            allData:[

            ],
            tableData: [],
        }
    },
    created()
    {
        this.getAllRecord();
    },
    computed:
    {
        refreshTableData()
        {
            var temData = this.allData;
            temData.forEach((item,index)=>{
              item.borrow_time = item.borrow_time.substr(0,10);
              item.return_time = item.return_time.substr(0,10);  
            })
            this.currentPage = 1;
            return temData;
        }
    },
    methods:
    {
        getReturnTime(row)
        {
            return row.is_return?row.return_time:'未归还';
        },
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
        getAllRecord()
        {
            this.$axios.get(
              this.$rust + '/managers/'+ JSON.parse(sessionStorage.getItem("user")||"{}").id +'/edit_record',
            )
            .then(res=>{
                console.log('返回数据成功',res);
                this.allData = res.data;

            })
            .catch(res=>{
            console.log('返回数据失败',res);
          })
        },
        ifMana(row)
        {
            return row.ifMana?'是':'否';
        }
    }
}
</script>

<style scoped>
    

</style>
