using Google.Protobuf;
using TestPhone;


Person person = new Person();
person.Id = 1;
person.Name = "laixhe";

person.Phones.Add(new Phone {
    Number = "130",
    TypeId = PhoneType.Work,
});

ContactBook contactBook = new ContactBook();
contactBook.Persons.Add(person);


Console.WriteLine(contactBook);

// 序列化操作
byte[] data = contactBook.ToByteArray();

// 反序列化操作
ContactBook contactBook1 = ContactBook.Parser.ParseFrom(data);

Console.WriteLine(BitConverter.ToString(data));
Console.WriteLine(contactBook1);