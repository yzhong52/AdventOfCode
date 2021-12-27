export function assertEqual(left: string, right: string) {
    if (left == right) {
        console.log(`✅ Equal:\n - ${left}\n - ${right}\n`)
    } else {
        console.error(`⛔ Equal:\n - ${left}\n - ${right}\n`)
    }
}