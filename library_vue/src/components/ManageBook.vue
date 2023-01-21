<template>
    <div>
        <el-row>
            <div style="width:100%">
                <el-input v-model="search" placeholder="输入书名进行搜索..." style="width:50%"/>
                <el-button type="primary" style="width:20%;margin-left: 30%;" @click="clickAdd">添加书籍</el-button>
            </div>
        </el-row>
        <el-table
            :data="refreshTableData.slice((currentPage-1)*pageSize,currentPage*pageSize)"
            style="width: 100%;margin-top:20px" border :height="580"
            :default-sort = "{prop: 'id', order: 'ascending'}" stripe empty-text="暂无数据" >
            <el-table-column  prop="id" label="书籍编号" min-width="1" sortable align="center"></el-table-column>
            <el-table-column prop="name" label="书名" min-width="1" align="center"></el-table-column>
            <el-table-column prop="writer" label="作者" min-width="1" align="center" ></el-table-column>
            <el-table-column prop="press" label="出版社" min-width="1" align="center"></el-table-column>
            <el-table-column label="状态" min-width="1" align="center" :formatter="getBorrowed" column-key="status" ></el-table-column>
            <el-table-column   min-width="1" align="center">
                <template #default="scope">
                    <el-button link type="primary" size="large" @click="show_info(scope.row)">
                        修改
                    </el-button>
                    <el-button link type="primary" size="large" @click="remove(scope.row)">
                        删除
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

        <el-dialog title="图书信息" v-model="is_show" width="30%">
            <el-form ref="changeForm" :model="curBook" label-width="80px" >
                <el-form-item label="编号">
                    <el-input  v-model="curBook.id" disabled ></el-input>
                </el-form-item>
                <el-form-item label="书名">
                    <el-input  v-model="curBook.name" ></el-input>
                </el-form-item>
                <el-form-item label="作者">
                    <el-input v-model="curBook.writer"></el-input>
                </el-form-item>
                <el-form-item label="出版社">
                    <el-input v-model="curBook.press"></el-input>
                </el-form-item>

            </el-form>
            <span slot="footer" style="text-align:center">
                <el-button type="primary" @click="correctBook">修 改</el-button>
                <el-button @click="is_show = false">取 消</el-button>
            </span>
        </el-dialog>


        <el-dialog title="添加书籍" v-model="is_add" width="30%" >
            <el-form ref="addForm" :model="newBook" label-width="80px" >
                <el-form-item label="书名">
                    <el-input  v-model="newBook.name" ></el-input>
                </el-form-item>
                <el-form-item label="作者">
                    <el-input v-model="newBook.writer"></el-input>
                </el-form-item>
                <el-form-item label="出版社">
                    <el-input v-model="newBook.press"></el-input>
                </el-form-item>
            </el-form>
            <span slot="footer">
                <el-button type="primary" @click="addBook">添 加</el-button>
                <el-button @click="is_add = false">取 消</el-button>
            </span>
        </el-dialog>
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
            user:{},
            curBook:
            {
                id: 0,
                name: '',
                writer: '',
                press: ''
            },
            newBook:
            {
                name: '',
                writer: '',
                press: '',
            },
            is_show: false,
            is_add: false
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

        show_info(row)
        {
            if(row.is_borrowed)
            {
                this.$message.error("书籍外借中，不可修改");
                return;
            }
            this.curBook.id = row.id;
            this.curBook.name = row.name;
            this.curBook.writer = row.writer;
            this.curBook.press = row.press;
            this.is_show = true;
        },
        remove(row)
        {
            if(row.is_borrowed)
            {
                this.$message.error("书籍外借中，不可修改");
                return;
            }
            let u = JSON.parse(sessionStorage.getItem("user")||"{}");
            this.$axios.delete(
                this.$rust + '/managers/'+ u.id + '/' + row.id  
            )
            .then(res=>{
                
                this.$message.success("修改成功");
                this.getAllBooks();
            })
            .catch(res=>{
                
            this.$message.error(res.response.data.error_message);
          })
        },
        addBook()
        {
            let u = JSON.parse(sessionStorage.getItem("user")||"{}");
            this.$axios.post(
                this.$rust + '/managers/'+ u.id + '/edit_book',
                this.newBook
            )
            .then(res=>{
                
                this.$message.success("添加成功");
                this.is_add = false;
                this.getAllBooks();
            })
            .catch(res=>{
                
                this.$message.error(res.response.data.error_message);
          })
        },
        clickAdd()
        {
            this.newBook = 
            {
                name: '',
                writer: '',
                press: ''
            }
            this.is_add = true;
        },
        correctBook()// **
        {
            let u = JSON.parse(sessionStorage.getItem("user")||"{}");
            this.$axios.put(
                this.$rust + '/managers/'+ u.id + '/edit_book',
                this.curBook
            )
            .then(res=>{
                
                this.$message.success("删除成功");
                this.is_show = false;
                this.getAllBooks();
            })
            .catch(res=>{
                
                this.$message.error(res.response.data.error_message);
            }
            )
        },
    }
}
</script>

<style scoped>




</style>
