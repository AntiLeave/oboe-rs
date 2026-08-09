#include "oboe/OboeExt.h"
#include "oboe/OboeExtensions.h"

namespace oboe {

bool OboeExt_isMMapSupported() {
    return OboeExtensions::isMMapSupported();
}

bool OboeExt_isMMapEnabled() {
    return OboeExtensions::isMMapEnabled();
}

int32_t OboeExt_setMMapEnabled(bool enabled) {
    return OboeExtensions::setMMapEnabled(enabled);
}

bool OboeExt_isMMapUsed(AudioStream *oboeStream) {
    return OboeExtensions::isMMapUsed(oboeStream);
}

}
