<template>
    <div>
        <el-table 
            :data="refreshTableData.slice((currentPage-1)*pageSize,currentPage*pageSize)"
            style="width: 100%" border  @filter-change="filterChange"
            :default-sort = "{prop: 'id', order: 'ascending'}" stripe height="660">
            <el-table-column min-width="1"  prop="id" label="用户编号"  sortable align="center"></el-table-column>
            <el-table-column min-width="1" prop="name" label="用户名"  align="center"></el-table-column>
            <el-table-column min-width="1" prop="password" label="密码" align="center" ></el-table-column>
            <el-table-column min-width="1" label="管理员"  align="center" :formatter="ifMana" column-key="mana"></el-table-column>
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
    name: "AllUserInfo",
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
        this.getAllUsers();
    },
    computed:
    {
        refreshTableData()
        {
            var temData = this.allData;
            this.currentPage = 1;
            return temData;
        }
    },
    methods:
    {
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
        getAllUsers()
        {
            this.$axios({
              methods:'GET',
              url:this.$rust + '/users/all',
            })
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
