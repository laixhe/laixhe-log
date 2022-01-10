using Microsoft.AspNetCore.Mvc;

namespace WebApi.Controllers
{

    [ApiController]                              // 声明控制器
    [Route("[controller]/[action]")]             // 设置路由
    public class UserController : ControllerBase // 定义 Api 控制器
    {

        private readonly ILogger<UserController> _logger;

        public UserController(ILogger<UserController> logger)
        {
            _logger = logger;
        }

        [HttpGet]
        public ActionResult<string> Get()
        {
            //[Route("get")]
            //[Produces("application/json")]
            //[Consumes("application/json", "multipart/form-data")]

            _logger.LogInformation($"maybe this log is provided by Serilog...");

            return "get user";
        }
    }
}
