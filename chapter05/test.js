const {echo,new_person,Person} = require("./pkg/mywasm.js")

let arr= ["a","b","c"];
echo(arr)

let person = new_person("鲤鱼纹",18)
console.log(person.get_name(),person.get_age())

let personNew = new Person()
console.log(personNew.get_name(),personNew.get_age())
console.log(personNew.name)
personNew.name = "update name"
console.log(personNew.name)