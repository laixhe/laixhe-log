using Microsoft.AspNetCore.Mvc;

namespace WebApi.Controllers
{

    [ApiController]
    [Route("[controller]")]
    public class UserController : ControllerBase
    {

        [HttpGet]
        [Route("get")]
        //[Produces("application/json")]
        //[Consumes("application/json", "multipart/form-data")]
        public string Get()
        {
            return "get user";
        }
    }
}
