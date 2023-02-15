package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"reflect"
	"strconv"
	"time"

	"github.com/olivere/elastic/v7"
)

var client *elastic.Client
var host = []string{
	"http://127.0.0.1:9200/",
}

type Test struct {
	Name      string   `json:"name"`
	Age       int      `json:"age"`
	Interests []string `json:"interests"`
	Date      string   `json:"date"`
	Mes       string   `json:"mes"`
}

func init() {

	errorLog := log.New(os.Stdout, "ES-ERROR: ", log.LstdFlags)
	//traceLog := log.New(os.Stdout, "ES-TRACE: ", log.LstdFlags)
	//infoLog := log.New(os.Stdout, "ES-INFO: ", log.LstdFlags)

	var err error

	client, err = elastic.NewClient(
		elastic.SetErrorLog(errorLog),
		//elastic.SetTraceLog(traceLog),
		//elastic.SetInfoLog(infoLog),
		elastic.SetSniff(false),
		elastic.SetHealthcheckInterval(10*time.Second),
		elastic.SetURL(host...))

	if err != nil {
		log.Fatalln("connect error:", err)
	}

	// ping 连接测试
	for _, v := range host {

		start := time.Now()

		info, code, err := client.Ping(v).Do(context.Background())
		if err != nil {
			log.Fatalln("ping error", err)
		}

		duration := time.Since(start)

		fmt.Printf("ping %s cost time: %v\n", v, duration)
		fmt.Printf("ping %s returned with code %d and version %s\n", v, code, info.Version.Number)

	}

}

// 下面是简单的 CURD ============================

// 创建 index 索引
// HTTP PUT /xxx
func CreateIndex(index, mapping string) {

	result, err := client.CreateIndex(index).BodyString(mapping).Do(context.Background())
	if err != nil {
		fmt.Println("CreateIndex err: ", err)
		return
	}

	fmt.Println("CreateIndex: ", result)

}

// 校验 index 索引是否存在
// HTTP HEAD /xxx
func IndexExists(index string) {

	exists, err := client.IndexExists(index).Do(context.Background())
	if err != nil {
		fmt.Println("IndexExists err: ", err)
		return
	}

	fmt.Println("IndexExists: ", exists)

}

// 删除 index 索引
// HTTP DELETE /xxx
func DelIndex(index string) {

	response, err := client.DeleteIndex(index).Do(context.Background())
	if err != nil {
		fmt.Println("DelIndex err: ", err)
		return
	}

	fmt.Println("DelIndex: ", response)

}

// 创建文档数据
// HTTP PUT /test/_doc/1
// {"name":"laixhe","age":18,"interests":["Sports","Music"],"date":"2019-04-07 14:50:02", "mes":"学习ElasticSearch"}
// {"name":"lai","age":19,"interests":["Cycling","Music"],"date":"2019-04-07 14:51:02", "mes":"ElasticSearch入门"}
// {"name":"laiki","age":20,"interests":["Cycling","Mountains"],"date":"2019-04-07 14:52:02", "mes":"入门Golang"}
func Create() {

	// 使用结构体
	e1 := Test{"laixhe", 18, []string{"Sports", "Music"}, "2019-04-07 14:51:01", "学习ElasticSearch"}
	put1, err := client.Index().
		Index("test").
		Id("1").
		BodyJson(e1).
		Do(context.Background())
	if err != nil {
		fmt.Println("create put1 err: ", err)
		return
	}
	fmt.Println("create put1: ", put1)

	// 使用字符串
	e2 := `{"name":"lai","age":19,"interests":["Cycling","Music"],"date":"2019-04-07 14:50:02", "mes":"ElasticSearch入门"}`
	put2, err := client.Index().
		Index("test").
		Id("2").
		BodyJson(e2).
		Do(context.Background())
	if err != nil {
		fmt.Println("create put2 err: ", err)
		return
	}
	fmt.Println("create put2: ", put2)

	//
	e3 := Test{"laiki", 20, []string{"Cycling", "Mountains"}, "2019-04-07 14:52:03", "入门Golang"}
	put3, err := client.Index().
		Index("test").
		Id("3").
		BodyJson(e3).
		Do(context.Background())
	if err != nil {
		fmt.Println("create put3 err: ", err)
		return
	}
	fmt.Println("create put3: ", put3)

}

// 批量插入文档数据
func Batch(index string, datas ...interface{}) {

	bulkRequest := client.Bulk()

	for i, data := range datas {
		doc := elastic.NewBulkIndexRequest().
			Index(index).
			Id(strconv.Itoa(i)).
			Doc(data)
		bulkRequest = bulkRequest.Add(doc)
	}

	response, err := bulkRequest.Do(context.TODO())
	if err != nil {
		fmt.Println("Batch err: ", err)
		return
	}

	failed := response.Failed()
	iter := len(failed)
	fmt.Printf("Batch error: %v, %v\n", response.Errors, iter)

}

// 修改文档数据
// HTTP POST /test/_doc/2/_update
// {"doc":{"age":28}}
func Update() {
	res, err := client.Update().
		Index("test").
		Id("2").
		Doc(map[string]interface{}{"age": 18}).
		Do(context.Background())
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Printf("update age %s\n", res.Result)

}

// 删除文档数据
// HTTP DELETE /test/_doc/1
func Delete() {

	res, err := client.Delete().
		Index("test").
		Id("1").
		Do(context.Background())
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Printf("delete result %s\n", res.Result)
}

// 查找 ID 文档数据
// HTTP GET /test/_doc/xxx
func GetID(id string) {

	//通过 id 查找
	get, err := client.Get().
		Index("test").
		Id(id).
		Do(context.Background())
	if err != nil {
		fmt.Println("GetID err: ", err)
		return
	}

	fmt.Println("GetID: ", get)

	if get.Found {
		data, _ := get.Source.MarshalJSON()
		fmt.Println("GetID: ", string(data))
	}

}

// 查看某文档是否存在,给定文档ID查询
// HTTP HEAD /test/_doc/xxx
func Exists(id string) {

	exist, err := client.Exists().
		Index("test").
		Id(id).
		Do(context.Background())
	if err != nil {
		fmt.Println("Exists err: ", err)
		return
	}

	fmt.Println("Exists: ", exist)
}

// 查看某 index 索引所有文档数据
// HTTP POST /test/_doc/_search
// HTTP POST /test/_search
// {"query":{"match_all":{}}}
// 默认返回 10 条数据
func QueryMatchAll(index string) {

	res, err := client.Search(index).Do(context.Background())
	if err != nil {
		fmt.Println("T---- QueryMatchAll err: ", err)
		return
	}

	var test Test
	for _, item := range res.Each(reflect.TypeOf(test)) { // 从搜索结果中取数据的方法
		t := item.(Test)
		fmt.Printf("T---- QueryMatchAll: %#v\n", t)
	}

}

// 查看某 index 索引所有文档数据，带分页
// HTTP POST /test/_doc/_search
// HTTP POST /test/_search
// {"query":{"match_all":{}},"from":0,"size":2}
func QueryMatchAllPage(index string) {

	res, err := client.Search(index).From(0).Size(2).Do(context.Background())
	if err != nil {
		fmt.Println("T---- QueryMatchAllPage err: ", err)
		return
	}

	var test Test
	for _, item := range res.Each(reflect.TypeOf(test)) { // 从搜索结果中取数据的方法
		t := item.(Test)
		fmt.Printf("T---- QueryMatchAllPage: %#v\n", t)
	}

}

// 条件查询
// HTTP POST /test/_search
// {"query":{"match":{"mes":"ElasticSearch"}}}
func Query_Mes(){

	query := elastic.NewBoolQuery()
	query.Must(elastic.NewMatchQuery("mes", "ElasticSearch"))
	//query.Filter(elastic.NewRangeQuery("age").Gt(19)) // 年龄大于 19 岁的

	res, err := client.Search("test").
		Type("_doc").
		Query(query).
		Do(context.Background())

	if err != nil {
		fmt.Println("T---- Query_Mes err: ", err)
		return
	}

	var test Test
	// 从搜索结果中取数据的方法
	for _, item := range res.Each(reflect.TypeOf(test)) {
		t := item.(Test)
		fmt.Printf("T---- Query_Mes: %#v\n", t)
	}

}

// 条件查询带排序
// HTTP POST /test/_search
// {"query":{"match":{"mes":"ElasticSearch"}},"sort":[{"age":{"order":"desc"}}]}
func QuerySort_Mes(){

	query := elastic.NewBoolQuery()
	query.Must(elastic.NewMatchQuery("mes", "ElasticSearch"))

	res, err := client.Search("test").
		Type("_doc").
		Query(query).
		Sort("age", false).
		Do(context.Background())

	if err != nil {
		fmt.Println("T---- QuerySort_Mes err: ", err)
		return
	}

	var test Test
	// 从搜索结果中取数据的方法
	for _, item := range res.Each(reflect.TypeOf(test)) {
		t := item.(Test)
		fmt.Printf("T---- QuerySort_Mes: %#v\n", t)
	}

}
// 获取某字段最大或最小(计算函数 min | max)
// HTTP POST /test/_search
// {"aggs": {"age_min": {"min": {"field": "age"} } } }
func AggsMinMax(){

	age_min := elastic.NewMinAggregation().Field("age")
	//age_max := elastic.NewMaxAggregation().Field("age")

	res, err := client.Search("test").
		Type("_doc").
		Aggregation("age_min", age_min).
		Do(context.Background())

	if err != nil {
		fmt.Println("T---- AggsMinMax err: ", err)
		return
	}

	data, is := res.Aggregations.Min("age_min")
	if is {
		fmt.Println("T---- AggsMinMax: ", *data.Value)
	}

}


/*
	查看索引配置 settings
	GET /test/_settings

	查看索引映射 mapping
	GET /test/_mapping

	关闭索引
	POST /test/_close

    聚合
	HTTP POST /test/_search

	// 对某字段进行分组统计(聚合函数 terms)
	{
		"aggs": {
			"group_by_count_age": {
				"terms": {
					"field": "age"
				}
			}
		}
	}

	// 对某字段进行计算统计(计算函数 stats)
	{
		"aggs": {
			"grades_count_age": {
				"stats": {
					"field": "age"
				}
			}
		}
	}

	// 获取某字段最大或最小(计算函数 min | max)
	{
		"aggs": {
			"age_min": {
				"min": {
					"field": "age"
				}
			}
		}
	}

    minAgg := elastic.NewMinAggregation().Field("age")
	rangeAgg := elastic.NewRangeAggregation().Field("age").AddRange(0,30).AddRange(30,60).Gt(60)


	build := client.Search(index).Type(type_).Pretty(true)

	minResult, err := build.Aggregation("minAgg", minAgg).Do(context.Background())
	rangeResult, err := build.Aggregation("rangeAgg", rangeAgg).Do(context.Background())
	if err != nil {
		panic(err)
	}

	minAggRes, _ := minResult.Aggregations.Min("minAgg")
	fmt.Printf("min: %v\n", *minAggRes.Value)

	rangeAggRes, _ := rangeResult.Aggregations.Range("rangeAgg")
	for _, item := range rangeAggRes.Buckets {
		fmt.Printf("key: %s, value: %v\n", item.Key, item.DocCount)
	}

	//短语搜索 搜索about字段中有 rock climbing
	matchPhraseQuery := elastic.NewMatchPhraseQuery("about", "rock climbing")
	res, err = client.Search("megacorp").Type("employee").Query(matchPhraseQuery).Do(context.Background())
	printEmployee(res, err)

	//分析 interests
	aggs := elastic.NewTermsAggregation().Field("interests")
	res, err = client.Search("megacorp").Type("employee").Aggregation("all_interests", aggs).Do(context.Background())
	printEmployee(res, err)

*/

func main() {

	// 创建 index 索引
	// 静态映射
	// 指定 settings 索引配置, number_of_shards 分片数(默认5), number_of_replicas 备份数(默认1)
	// 指定 mappings 映射里面有一个名称为 test 的 Type
	// 指定映射 properties 属性有三个字段 user age interests
	var mapping = `{
	"settings":{
		"number_of_shards": 3,
		"number_of_replicas": 1
	},
	"mappings":{
		"_doc":{
			"properties":{
				"user":{
					"type":"text"
				},
				"age":{
					"type": "integer"
				},
				"interests":{
					"type":"keyword"
				},
				"date":{
					"type":"date",
					"format":"yyyy-MM-dd HH:mm:ss||epoch_millis"
				},
				"mes":{
					"type":"text"
				}
			}
		}
	}
}`
	// epoch_millis 时间戳

	// include_type_name=false

	// 创建 index 索引
	//CreateIndex("test", mapping)
	_ = mapping
	//IndexExists("test")
	//DelIndex("test")

	//Create()
	//Update()
	//GetID("1")
	//Exists("2")
	//QueryMatchAll("test")
	//QueryMatchAllPage("test")
	//Query_Mes()
	//QuerySort_Mes()
	//AggsMinMax()

}
