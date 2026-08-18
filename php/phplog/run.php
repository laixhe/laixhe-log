<?php

/**
 * phplog 主入口：串联所有基础类型、集合、流、时间、并发、JSON、HTTP 示例。
 * 对应 Rust rustlog/src/main.rs（cargo run）与 Go golog 的测试用例（go test -v）。
 */

declare(strict_types=1);

require __DIR__ . '/vendor/autoload.php';

use Laixhe\Phplog\CharStringDemo;
use Laixhe\Phplog\CollectionDemo;
use Laixhe\Phplog\HttpDemo;
use Laixhe\Phplog\JsonDemo;
use Laixhe\Phplog\NumberDemo;
use Laixhe\Phplog\StreamDemo;
use Laixhe\Phplog\SyncDemo;
use Laixhe\Phplog\TimeDemo;

// ===== number：数值类型（对应 Rust number.rs）=====
echo '============= number::number_to_string =============', PHP_EOL;
NumberDemo::numberToString();
echo '============= number::overflow =============', PHP_EOL;
NumberDemo::overflow();
echo '============= number::type_conversion =============', PHP_EOL;
NumberDemo::typeConversion();

// ===== char_string：字符与字符串（对应 Rust char_string.rs）=====
echo '============= char_string::std_char =============', PHP_EOL;
CharStringDemo::stdChar();
echo '============= char_string::std_string =============', PHP_EOL;
CharStringDemo::stdString();
echo '============= char_string::string_parse =============', PHP_EOL;
CharStringDemo::stringParse();

// ===== collection：集合类型（对应 Rust array_map.rs）=====
echo '============= collection::std_array =============', PHP_EOL;
CollectionDemo::stdArray();
echo '============= collection::std_assoc_array =============', PHP_EOL;
CollectionDemo::stdAssocArray();
echo '============= collection::std_deque =============', PHP_EOL;
CollectionDemo::stdDeque();
echo '============= collection::std_queue_stack =============', PHP_EOL;
CollectionDemo::stdQueueStack();
echo '============= collection::std_priority_queue =============', PHP_EOL;
CollectionDemo::stdPriorityQueue();
echo '============= collection::ring_buffer =============', PHP_EOL;
CollectionDemo::ringBuffer();
echo '============= collection::lru_cache =============', PHP_EOL;
CollectionDemo::lruCache();
echo '============= collection::slice_handle =============', PHP_EOL;
CollectionDemo::sliceHandle();
echo '============= collection::map_handle =============', PHP_EOL;
CollectionDemo::mapHandle();
echo '============= collection::unique =============', PHP_EOL;
CollectionDemo::unique();
echo '============= collection::grouping =============', PHP_EOL;
CollectionDemo::grouping();

// ===== stream：流/数组函数专题（对应 Rust iterators.rs）=====
echo '============= stream::basics =============', PHP_EOL;
StreamDemo::basics();
echo '============= stream::adapters =============', PHP_EOL;
StreamDemo::adapters();
echo '============= stream::consumers =============', PHP_EOL;
StreamDemo::consumers();
echo '============= stream::practice (综合实战) =============', PHP_EOL;
StreamDemo::practice();

// ===== time：时间处理（对应 Go time_test.go）=====
echo '============= time::time_basics =============', PHP_EOL;
TimeDemo::timeBasics();
echo '============= time::time_parse =============', PHP_EOL;
TimeDemo::timeParse();
echo '============= time::time_before_after =============', PHP_EOL;
TimeDemo::timeBeforeAfter();
echo '============= time::time_since_until =============', PHP_EOL;
TimeDemo::timeSinceUntil();
echo '============= time::time_zone =============', PHP_EOL;
TimeDemo::timeZone();

// ===== sync：并发同步（对应 Go sync_test.go）=====
echo '============= sync::once =============', PHP_EOL;
SyncDemo::once();
echo '============= sync::wait_group =============', PHP_EOL;
SyncDemo::waitGroup();
echo '============= sync::fiber_schedule =============', PHP_EOL;
SyncDemo::fiberSchedule();
echo '============= sync::mutex =============', PHP_EOL;
SyncDemo::mutex();
echo '============= sync::atomic =============', PHP_EOL;
SyncDemo::atomic();

// ===== json：JSON 序列化（对应 Go json_test.go）=====
echo '============= json::json_basic =============', PHP_EOL;
JsonDemo::jsonBasic();
echo '============= json::json_query =============', PHP_EOL;
JsonDemo::jsonQuery();
echo '============= json::json_pretty =============', PHP_EOL;
JsonDemo::jsonPretty();
echo '============= json::json_errors =============', PHP_EOL;
JsonDemo::jsonErrors();

// ===== http：HTTP 服务端与客户端（对应 Go http_serve/http_client）=====
echo '============= http::http_server + client =============', PHP_EOL;
HttpDemo::httpDemo();

echo '============= done =============', PHP_EOL;
