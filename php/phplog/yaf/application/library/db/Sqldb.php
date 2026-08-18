<?php
namespace db;

use PDO;
use PDOException;
use Log;
use Yaf\Registry;

/**
 * 数据库操作处理
 */
class Sqldb {

    // 用于单例存放本对象
    private static $instance;
    // 用于存放数据库的联接资源
    private $db = null;
    // sql操作
    private $statement = null;

    /**
     * 错误信息
     */
    private static $error = [];

    /**
     * 用构造函数初始化数据库的连接信息,并进行私有化和最终
     */
    final protected function __construct(){

        // 获取配置文件的信息，返回对象
        $default_db = Registry::get('config');

        $driver   = $default_db->database->default->driver;       // 数据库类型
        $hostname = $default_db->database->default->hostname;     // 服务器地址
        $port     = $default_db->database->default->port;         // 端口
        $database = $default_db->database->default->database;     // 数据库名
        $username = $default_db->database->default->username;     // 用户名
        $password = $default_db->database->default->password;     // 密码
        $charset  = $default_db->database->default->charset;      // 数据库编码默认采用 utf8mb4

        try {

            switch ($driver){
                case 'mysql':

                    //mysql:host=127.0.0.1;dbname=test;port=3306;charset=utf8mb4
                    // 组合连接 数据库类型 数据库地址 数据库名称 数据库端口 数据库通信字符集
                    $hsdb = "mysql:host={$hostname};dbname={$database};port={$port};charset={$charset}";

                    // 用于连接mysql数据库
                    $this->db = new PDO($hsdb, $username, $password);

                    break;

                case 'sqlite':

                    // 组合连接 数据库类型 数据库地址
                    $hsdb = 'sqlite:'.$hostname;
                    // 用于连接sqlite数据库
                    $this->db = new PDO($hsdb);

                    self::$error[] = 'connect sqlite:'.$hostname;
                    break;

                default:

                    trigger_error('请配置数据库正确的类型：mysql、sqlite', E_USER_ERROR);

            }

            if(!is_null($this->db)){
                // 设置错误报告为抛出异常模式
                $this->db->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
            }

        } catch (PDOException $e) {

            trigger_error('connect sql error：' . $e->getMessage(), E_USER_ERROR);

        }

    }

    /**
     * 进行单例模式
     */
    public static function getDb() : Sqldb {
        // 判断自身的单例对象实例是否是自身的实例(单例模式)
        if (! (self::$instance instanceof self)) {
            self::$instance = new self();
        }
        return self::$instance;
    }

    /**
     * 防止被克隆,并进行私有化和最终
     */
    final protected function __clone(){

    }
    // 拦截器
    public function __set($name, $value){
        return false;
    }

    /**
     * 发送执行
     *
     * @access public
     * @param string $sql sql语句
     * @param array $arr  用于预处理(一\二维的关联数组)
     * @param bool $insid 用于是否要有'最后插入的自增ID'(默认false)
     * @return int        返回执行成功后的影响行数|自增ID
     */
    public function query(string $sql, array $arr = [], bool $insid = false) : int {
        try {
            // 进行预处理的准备查询语句
            $this->statement = $this->db->prepare($sql);

            // 累计影响行数（批量执行时逐条累加）
            $affectedRows = 0;

            // 判断有没有预处理数据
            if (empty($arr)) {
                $this->statement->execute();
                $affectedRows = $this->statement->rowCount();
            } else {
                if (isset($arr[0]) && is_array($arr[0])) {
                    // 二维数组：批量执行，逐条累加影响行数
                    foreach ($arr as $v) {
                        $this->statement->execute($v);
                        $affectedRows += $this->statement->rowCount();
                    }
                } else {
                    // 一维数组：单次执行
                    $this->statement->execute($arr);
                    $affectedRows = $this->statement->rowCount();
                }
            }

            // 最后插入ID
            if ($insid) {
                return intval($this->db->lastInsertId());
            }

            // 返回执行成功后的影响行数
            return $affectedRows;

        } catch (PDOException $e) {

            // 写入错误
            self::$error[] = 'query sql error：' . $e->getMessage();
            self::$error[] = $sql;
        }

        return 0;
    }

    /**
     * 获取一行数据
     *
     * @access public
     * @param string $sql sql语句
     * @param array $arr  用于预处理(一维的关联数组)
     * @param bool $fetch 用于返回结果的方式 true为关联数组(默认) false为索引数组
     * @return array
     */
    public function fetch(string $sql, array $arr = [], bool $fetch = true) : array {
        try {
            // 进行预处理的准备查询语句
            $this->statement = $this->db->prepare($sql);
            // 判断有没有预处理数据
            if (empty($arr)) {
                $this->statement->execute();
            } else {
                $this->statement->execute($arr);
            }
            // 判断返回数据的方法
            if ($fetch) {

                $data = $this->statement->fetch(PDO::FETCH_ASSOC);
                if (is_array($data)){
                    return $data;
                }

            } else {

                $data = $this->statement->fetch(PDO::FETCH_NUM);
                if (is_array($data)){
                    return $data;
                }

            }
        } catch (PDOException $e) {

            // 写入错误
            self::$error[] = 'fetch sql error：' . $e->getMessage();
            self::$error[] = $sql;
        }

        return [];
    }

    /**
     * 获取多行数据
     *
     * @access public
     * @param string $sql sql语句
     * @param array $arr  用于预处理(一维的关联数组)
     * @param bool $fetch 用于返回结果的方式 true为关联数组(默认) false为索引数组
     * @return array
     */
    public function fetchAll(string $sql, array $arr = [], bool $fetch = true) : array {
        try {
            // 进行预处理的准备查询语句
            $this->statement = $this->db->prepare($sql);
            // 判断有没有预处理数据
            if (empty($arr)) {
                $this->statement->execute();
            } else {
                $this->statement->execute($arr);
            }

            // 判断返回数据的方法
            if ($fetch) {

                $data = $this->statement->fetchAll(PDO::FETCH_ASSOC);
                if (is_array($data)){
                    return $data;
                }

            } else {

                $data = $this->statement->fetchAll(PDO::FETCH_NUM);
                if (is_array($data)){
                    return $data;
                }

            }
        } catch (PDOException $e) {

            // 写入错误
            self::$error[] = 'fetchAll sql error：' . $e->getMessage();
            // 记录sql语句
            self::$error[] = $sql;
        }

        return [];
    }

    /**
     * 开启事务
     */
    public function begin() : bool {
        try {
            return $this->db->beginTransaction();
        } catch (PDOException $e) {
            self::$error[] = 'begin transaction error：' . $e->getMessage();
            return false;
        }
    }

    /**
     * 提交事务
     */
    public function commit() : bool {
        try {
            return $this->db->commit();
        } catch (PDOException $e) {
            self::$error[] = 'commit transaction error：' . $e->getMessage();
            return false;
        }
    }

    /**
     * 回滚事务
     */
    public function rollBack() : bool {
        try {
            return $this->db->rollBack();
        } catch (PDOException $e) {
            self::$error[] = 'rollBack transaction error：' . $e->getMessage();
            return false;
        }
    }

    /**
     * 获取自增ID
     */
    public function insertId() : int {
        return intval($this->db->lastInsertId());
    }

    /**
     * 获取错误信息
     */
    public static function getError() : array {
        return self::$error;
    }

    /**
     * 析构方法用于释放数据库连接资源和sql操作
     */
    public function __destruct(){
        $this->statement = null;
        $this->db = null;

        if (!empty(self::$error)){

            foreach (self::$error as $v){
                Log::error($v);
            }

            self::$error = [];
        }

    }
}
