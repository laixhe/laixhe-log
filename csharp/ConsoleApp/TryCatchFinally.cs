using System;

// 异常捕获机制
/*
try{ 
  可能出现的代码块
}catch(Exception e){ 
  出错之后执行的代码块
} finally {
  不管有没有出错都执行的代码块
}

throw new Error("主动抛出异常");
*/

namespace ConsoleApp {
    class TryCatchFinally {
        public static void run() {
            
            try{
                throw new Exception("error test");
            } catch(Exception e) {
                Console.WriteLine("{0}", e);
            }
            
            Console.WriteLine("TryCatchFinally.run()");
        }
    }
}