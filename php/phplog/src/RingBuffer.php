<?php

namespace Laixhe\Phplog;

/**
 * 环形缓冲区（对应 Go container/ring）：固定容量环，写满覆盖最旧数据。
 */
final class RingBuffer
{
    private array $buf;
    private int $head = 0;
    private int $size = 0;
    private int $capacity;

    public function __construct(int $capacity)
    {
        $this->capacity = $capacity;
        $this->buf = array_fill(0, $capacity, null);
    }

    // 写入：环满则覆盖最旧数据
    public function add(mixed $value): void
    {
        $this->buf[$this->head] = $value;
        $this->head = ($this->head + 1) % $this->capacity;
        if ($this->size < $this->capacity) {
            $this->size++;
        }
    }

    // 从最旧到最新遍历
    public function snapshot(): array
    {
        $list = [];
        $start = ($this->head - $this->size + $this->capacity) % $this->capacity;
        for ($i = 0; $i < $this->size; $i++) {
            $list[] = $this->buf[($start + $i) % $this->capacity];
        }
        return $list;
    }

    public function size(): int
    {
        return $this->size;
    }
}
