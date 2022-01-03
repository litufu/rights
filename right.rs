
//权利
struct Right{
    id:String,//权利唯一标识
    name: String,//权利名称
    version: String,//版本
    range:String//权利的范围、组织内、组织外

    sourceType:RightSouceType,//权利来源的类型
    source: String,//权利的来源，权利的创建者、规章规定、授权者
    sourceId:String,//权利来源的唯一标识

    startTime:String,//权利开始时间
    endTime:String,//权利的结束时间

    oppositeSideId:String,//权利的对方的唯一标识
    oppositeSideName:String,//权利的对方，什么物质、什么人或组织
    oppositeSideType:RightOppositeSideType,//权利的对方的类型

    ownerId:String,//权利所有者,一个人所有、多个人（组织）所共有。
    ownerType:String,//权利所有者类型
    ownerName:String//权利所有者名称
}

enum OwnerType{
    Person,//个人
    Orgnization,//组织
}

//权利的来源类型
enum RightSouceType {
    Create,//创立、组织创立者享有组织权利来源
    Rules,//规章制度、通过规章制度赋予组织内人员权利
    Grant，//直接授予人员组织内权利
    Contract,//组织外部权利来源
}

//权利的对象类型
enum RightOppositeSideType{
    Substance,//对物质的权利、表现为物权
    Person,//对人的权利
    Orgnization,//对其他组织的权利
}

//物权的分类
enum RealRight{
    Own,//所有权
    Use,//使用权
    Guarantee,//担保权
}

impl Right{

    //权利的创建，与权利的取消相对应
    fn create(&self) -> Right{
        return Right{};
    }


    //将权利授予某人、某个组织。与权利的回收相对应。
    fn grant(&self){

    }

    //将授予的权利收回、授予者自然用友收回的权利
    fn recovery(&self){

    }

    //将权利取消掉
    fn cancel(&self){

    }

}



