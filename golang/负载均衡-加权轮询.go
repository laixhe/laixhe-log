package main

type WeightRoundRobinBalance struct {
	//curIndex int
	rss []*WeightNode
	//rsw      []int

	conf LoadBalanceConf // 观察主体
}

// LoadBalanceConf 配置主题
type LoadBalanceConf interface {
	GetConf() []string
	WatchConf()
	UpdateConf(conf []string)
}

type WeightNode struct {
	addr            string // 服务器地址
	weight          int    // 权重值
	currentWeight   int    // 节点当前权重
	effectiveWeight int    // 有效权重
}

func (r *WeightRoundRobinBalance) Add(addr string, weight int) {
	if weight <= 0 {
		weight = 1
	}
	node := &WeightNode{addr: addr, weight: weight}
	node.effectiveWeight = node.weight
	r.rss = append(r.rss, node)
}

func (r *WeightRoundRobinBalance) Next() string {
	total := 0
	var best *WeightNode
	for i := 0; i < len(r.rss); i++ {
		w := r.rss[i]
		// step 1 统计所有有效权重之和
		total += w.effectiveWeight

		// step 2 变更节点临时权重为的节点临时权重+节点有效权重
		w.currentWeight += w.effectiveWeight

		// step 3 有效权重默认与权重相同，通讯异常时-1, 通讯成功+1，直到恢复到weight大小
		if w.effectiveWeight < w.weight {
			w.effectiveWeight++
		}
		// step 4 选择最大临时权重点节点
		if best == nil || w.currentWeight > best.currentWeight {
			best = w
		}
	}
	if best == nil {
		return ""
	}
	// step 5 变更临时权重为 临时权重-有效权重之和
	best.currentWeight -= total
	return best.addr
}

func (r *WeightRoundRobinBalance) Get(key string) (string, error) {
	return r.Next(), nil
}

func (r *WeightRoundRobinBalance) SetConf(conf LoadBalanceConf) {
	r.conf = conf
}
