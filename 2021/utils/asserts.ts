export function assertEqual<T>(left: T, right: T) {
    if (left == right) {
        console.log(`✅ Equal:\n - ${left}\n - ${right}\n`)
    } else {
        console.error(`⛔ Equal:\n - ${left}\n - ${right}\n`)
    }
}
