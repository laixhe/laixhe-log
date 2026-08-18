#include <iostream>

#include "StdBits.h"
#include "StdCallback.h"
#include "StdCast.h"
#include "StdAlgorithm.h"
#include "StdArray.h"
#include "StdBasic.h"
#include "StdIO.h"
#include "StdIter.h"
#include "StdClass.h"
#include "StdContainer.h"
#include "StdControl.h"
#include "StdEnum.h"
#include "StdException.h"
#include "StdFile.h"
#include "StdFunction.h"
#include "StdJson.h"
#include "StdMove.h"
#include "StdNet.h"
#include "StdNumber.h"
#include "StdOptional.h"
#include "StdRange.h"
#include "StdRandom.h"
#include "StdRegex.h"
#include "StdSetOps.h"
#include "StdString.h"
#include "StdStringHandle.h"
#include "StdStruct.h"
#include "StdTemplate.h"
#include "StdThread.h"
#include "StdConst.h"
#include "StdPtr.h"
#include "Chrono.h"

int main()
{
    // StdString s{};
    // StdConst::ConstPointer();
    // StdPtr p{};  // 未包含头文件，可自行添加

    Chrono chrono{};

    // ===== 基础主题（参考 Go golog / Rust rustlog 的基础用例）=====
    StdBasic basic{};
    StdControl control{};
    StdFunction function{};
    StdClass classDemo{};
    StdTemplate templateDemo{};
    StdFile file{};
    StdException exception{};
    StdArray array{};
    StdEnum enumDemo{};
    StdAlgorithm algorithm{};
    StdRegex regex{};
    StdMove moveDemo{};
    StdStringHandle stringHandle{};
    StdSetOps setOps{};
    StdNet net{};
    StdOptional optional{};
    StdRandom random{};
    StdBits bits{};
    StdIO io{};
    StdCallback callback{};
    StdStruct structDemo{};
    StdCast cast{};
    StdIter iter{};

    // ===== 进阶主题（参考 Go golog / Rust rustlog）=====
    StdNumber number{};
    StdContainer container{};
    StdRange range{};
    StdThread threadDemo{};
    StdJson jsonDemo{};

    return 0;
}
