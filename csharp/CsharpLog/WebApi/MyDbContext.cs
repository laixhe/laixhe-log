using Microsoft.EntityFrameworkCore;
using WebApi.Models;

namespace WebApi
{
    // 创建数据库上下文
    public class MyDbContext : DbContext
    {
        public DbSet<User> user { get; set; }

        public MyDbContext(DbContextOptions<MyDbContext> options) : base(options)
        {
        }
    }
}
