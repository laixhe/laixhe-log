using System;
using System.IO;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace CsharpLog
{
    class JsonNew {
        public static void Run() {

            var data = new TestJsonData{
                a = 10.1,
                b = true
            };

            var testJson = new TestJson{
                name = "中文",
                id = 18,
                damage = 800.8,
                arrayInt = new int[]{1,2,3},
                data = data
            };

            string testJsonStr = JsonSerializer.Serialize(testJson, typeof(TestJson));
            Console.WriteLine("{0}", testJsonStr);
            // 结果： {"name":"\u4E2D\u6587","id":18,"damage":800.8,"array_int":[1,2,3],"data":{"a":10.1,"b":true}}

            var testJsonObj = JsonSerializer.Deserialize<TestJson>(testJsonStr);
            Console.WriteLine("{0}", testJsonObj);
        }
    }

    public class TestJson{
        public string name{get; set;}
        public int id{get; set;}
        public double damage{get; set;}
        [JsonPropertyName("array_int")]
        public int[] arrayInt{get; set;}
        public TestJsonData data {get; set; }

        public override string ToString(){
            return string.Format("name:{0}, id:{1}, damage:{2}, data:{3}", 
            name, 
            id, 
            damage, 
            data);
        }
    }

    public class TestJsonData{
        public double a {get; set; }
        public bool b {get; set; }

        public override string ToString(){
            return string.Format("a:{0}, b:{1}", 
            a, 
            b);
        }
    }
}