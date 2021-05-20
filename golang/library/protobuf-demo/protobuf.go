package main

import (
	"fmt"

	"google.golang.org/protobuf/proto"

	"golang_log/library/protobuf-demo/test"
)

func GoogleProtobuf() {

	// 联系簿
	contactBook := new(test.ContactBook)

	for i := 0; i < 60; i++ {

		// 每人
		p := &test.Person{
			Id:   int32(i),
			Name: fmt.Sprintf("lai-%d", i),
		}

		// 手机
		phone := &test.Phone{
			TypeId: test.PhoneType_HOME,
			Number: "10086",
		}

		// 每人的手机
		p.Phones = append(p.Phones, phone)

		// 联系簿
		contactBook.Persons = append(contactBook.Persons, p)
	}

	fmt.Println(contactBook)
	fmt.Println("---------------------------------------------")

	// 下面进行编解码

	// 进行编码 Protobuf
	data, err := proto.Marshal(contactBook)
	if err != nil {
		fmt.Println("proto.Marshal err: ", err)
		return
	}

	cBook := new(test.ContactBook)

	// 进行解码 Protobuf
	err = proto.Unmarshal(data, cBook)
	if err != nil {
		fmt.Println("proto.Unmarshal err: ", err)
		return
	}

	fmt.Println(cBook)

}
