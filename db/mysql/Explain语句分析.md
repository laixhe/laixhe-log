### 作用

- 1. 表的**读取顺序**（**id** 字段，越大优先级越高）
- 2. 数据读取操作的**操作类型**（**select_type** 字段）
- 3. 哪些**索引可以使用**（**possible_keys** 字段）
- 4. 哪些**索引被实际使用**（**keys** 字段）
- 5. 表之间的引用（ref 字段）
- 6. 每张表有**多少行被优化器查询**（**rows** 字段）

#### id
- `id相同`，**执行顺序由上至下**
- `id不同`，如果是子查询，id的序号会递增，**id值越大优先级越高，越先被执行**

#### select_type
> 查询的类型，主要用于区别普通查询、联合查询、子查询等复杂查询

- 1. **SIMPLE**：简单的select查询，查询中不包含子查询或者UNION
- 2. **PRIMARY**：查询中若包含任何复杂的子部分，最外层查询则被标记为PRIMARY
- 3. **SUBQUERY**：在SELECT或者WHERE列表中包含了**子查询**
- 4. **DERIVED**：在FROM列表中包含的子查询被标记为DERIVED（**衍生**）MySQL会递归执行这些子查询，把结果放在临时表里
- 5. **UNION**：若第二个SELECT出现在UNION之后，则被标记为UNION；若UNION包含在FROM子句的子查询中，外层SELECT将被标记为：DERIVED
- 6. **UNION RESUL**T：从UNION表获取结果的SELECT

#### type
> 访问类型排列，显示查询使用了何种类型

- **system**：**表只有一行记录（等于系统表）**，这是const类型的特例，平时不会出现，这个也可以忽略不计

- **const**：表示**通过索引一次就能找到了**，const用于**比较primary key或者unique索引**。因为只匹配一行数据，所以很快。如将主键置于where列表中，MySQL就能将该查询转换为一个常量

- **eq_ref**：**唯一性索引**，对于每个索引建，表中**只有一条记录与之匹配**，常见于主键或唯一索引**扫描**

- **ref**：**非唯一索引扫描**，返回匹配某个单独值的所有行。本质上也是一种索引访问，它返回所有匹配某个单独值的行。不过，它可能找到多个符合条件的行，所以它应该**属于查找和扫描的混合体**

- **range**：只检索给定范围的行，使用一个索引来选择行。key列显示使用了哪个索引，一般就是你的**where语句中出现了between、<、>、in**等的查询，这种范围扫描索引比全表扫描要好，因为他只需要开始索引的某一个点，而结束于另一点，不用扫描全部索引

- **index**：Full Index Scan，**index与ALL区别为index类型只遍历索引树**。这通常比ALL快，因为索引文件通常比数据文件小。(也就是说虽然all和index都是读全表，但index是从索引中读取的，而all是从硬盘数据库文件中读的)

- **ALL**：Full Table Scan，将遍历全表以找到匹配的行(**全表扫描**)

> 从好到坏依次是: system > const > eq_ref > ref > fultext > ref_or_null > index_merge > unique_subquery > index_subquery > range > index > ALL
> 
> 重要的指标为：system > const > eq_ref > ref > range > index > ALL

#### possible_keys
- 显示**可能应用在这张表上的索引**，一个或多个
- 若查询涉及的字段上存在索引，则该索引将被列出，但不一定被查询实际使用

#### key

- **实际使用的索引**，如果为null，则没有使用索引
- 若查询中使用了覆盖索引，则该索引仅出现在key列表中

#### key_len
- 表示**索引中使用的字节数**，可通过该列计算查询中使用的索引的长度。**在不损失精确性的情况下，长度越短越好**
- **key_len显示的值为索引最大可能长度**，并非实际使用长度，即key_len是根据表定义计算而得，不是通过表内检索出的

#### ref
- 显示**索引哪一列被使用了**，如果可能的话，**最好是一个常数**。哪些列或常量被用于查找索引列上的值

#### rows
- 根据表统计信息及索引选用情况，大致估算出**找到所需的记录所需要读取的行数**

#### Extra
- 1. Using filesort(文件排序)
  - 在使用 order by 关键字的时候，如果待排序的内容不能由所使用的索引直接完成排序的话，那么 mysql 有可能就要进行文件排序 
  
- 2. Using temporary(创建临时表)
  - 一般出现在**多张表的数据需要排序**的情况下 MySQl 会先使用 using temporary 保存临时数据, 然后再在临时表上使用 filesort 进行排序
  - 如果有 ORDER BY 子句和一个不同的 GROUP BY 子句, 或者如果 ORDER BY 或 GROUP BY 中的字段都来自其他的表而非连接顺序中的第一个表的话, 就会创建一个临时表了

- 3. Using indexing(覆盖索引)
  - 表示相应的 select 操作中使用了覆盖索引（Coveing Index），避免访问了表的数据行。
  - 如果**同时出现 using where**，表明索引被**用来执行索引键值的查找**
  - 如果**没有**同时出现 using where，**表明索引用来读取数据**而非执行查找动作

- 4. Using where
  - 表示使用了 where 过滤

- 5. Using join buffer
  - 表示使用了连接缓存

- 6. impossible where
  - where 子句的值总是 false，不能用来获取任何元组

- 7. select tables optimized away
  - 在没有 GROUP BY 子句的情况下，基于索引优化 MIN/MAX 操作或者对于 MyISAM 存储引擎优化 COUNT(*) 操作，不必等到执行阶段再进行计算，查询执行计划生成的阶段即完成优化

- 8. distinct
  - 优化 distinct，在找到第一匹配的元组后即停止找同样值的工作

