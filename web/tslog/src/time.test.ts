import { test, expect } from "bun:test";
// date-fns：流行的日期处理库，提供格式化、解析、比较等函数。
// 注意格式 token 大小写：yyyy=年、MM=月、dd=日、HH=时(24h)、mm=分、ss=秒。
import { format, getUnixTime, fromUnixTime, parse, isBefore, differenceInSeconds } from "date-fns";

// 固定基准时间：2026-08-13 12:13:14（本地时间），用于稳定断言
const BASE = new Date(2026, 7, 13, 12, 13, 14);

test("获取当前时间戳（秒级）", () => {
  const seconds = getUnixTime(new Date());
  expect(seconds).toBeGreaterThan(0);
  // 与毫秒时间戳一致（误差在 1 秒内）
  expect(Math.abs(seconds * 1000 - Date.now())).toBeLessThan(1000);
});

test("当前时间格式化为 yyyy-MM-dd HH:mm:ss", () => {
  const s = format(new Date(), "yyyy-MM-dd HH:mm:ss");
  // 校验长度与分隔符
  expect(s.length).toBe(19);
  expect(s[4]).toBe("-");
  expect(s[7]).toBe("-");
  expect(s[10]).toBe(" ");
  expect(s[13]).toBe(":");
  expect(s[16]).toBe(":");
});

test("时间戳转时间对象", () => {
  const seconds = getUnixTime(BASE); // 秒级时间戳
  const d = fromUnixTime(seconds); // 转回 Date 对象
  expect(d.getFullYear()).toBe(2026);
  expect(d.getMonth() + 1).toBe(8); // getMonth 从 0 开始
  expect(d.getDate()).toBe(13);
  expect(d.getHours()).toBe(12);
  expect(d.getMinutes()).toBe(13);
  expect(d.getSeconds()).toBe(14);
});

test("格式化固定时间", () => {
  expect(format(BASE, "yyyy-MM-dd HH:mm:ss")).toBe("2026-08-13 12:13:14");
});

test("时间字符串解析为时间对象", () => {
  // parse(字符串, 格式, 参考日期)：按指定格式解析为 Date
  const d = parse("2026-08-13 12:13:14", "yyyy-MM-dd HH:mm:ss", new Date());
  expect(format(d, "yyyy-MM-dd HH:mm:ss")).toBe("2026-08-13 12:13:14");
  expect(getUnixTime(d)).toBe(getUnixTime(BASE));
});

test("时间比较", () => {
  const t1 = parse("2026-08-13 12:13:14", "yyyy-MM-dd HH:mm:ss", new Date());
  const t2 = parse("2026-08-13 12:13:15", "yyyy-MM-dd HH:mm:ss", new Date());

  // isBefore：判断前者是否早于后者
  expect(isBefore(t1, t2)).toBe(true);
  // differenceInSeconds：两个时间相差的秒数（正数表示 t2 更晚）
  expect(differenceInSeconds(t2, t1)).toBe(1);
});
