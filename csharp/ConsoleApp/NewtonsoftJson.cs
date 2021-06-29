using System;
using Newtonsoft.Json;

namespace ConsoleApp {
    class NewtonsoftJson {
        public static void run() {

            var data = new TestNewtonsoftJsonData{
                a = 10.1,
                b = true
            };

            var testJson = new TestNewtonsoftJson{
                data = data,
                id = 18,
                name = "中文"
            };

            string testJsonStr =JsonConvert.SerializeObject(testJson);
            Console.WriteLine("{0}", testJsonStr);
            // 结果： {"data":{"a":10.1,"b":true},"id":18,"name":"中文"}

            var testJsonObj = JsonConvert.DeserializeObject<TestNewtonsoftJson>(testJsonStr);
            Console.WriteLine("{0}", testJsonObj);

        }
    }

    public class TestNewtonsoftJson{
        public TestNewtonsoftJsonData data {get; set; }
        public int id {get; set; }
        public string name {get; set; }

        public override string ToString(){
            return string.Format("id:{0}, name:{1}, data:{2}", 
            id, 
            name,
            data);
        }
    }

    public class TestNewtonsoftJsonData{
        public double a {get; set; }
        public bool b {get; set; }

        public override string ToString(){
            return string.Format("a:{0}, b:{1}", 
            a, 
            b);
        }
    }
}