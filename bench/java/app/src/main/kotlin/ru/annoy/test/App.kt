/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package ru.annoy.test

import com.github.hanabi1224.RuAnnoy.*
import com.spotify.annoy.ANNIndex

fun main(args: Array<String>) {
    val dim = args[0].toInt()
    val size = args[1].toInt()
    var nResult = args[2].toInt()
    var nLoop = args[3].toLong()

    for (metric in arrayOf("angular", "euclidean")) {
        val path = "../../index.$metric.${dim}d.ann"
        // println("Testing index $path")
        val index1 = AnnoyIndex.tryLoad(path, dim, IndexType.valueOf(metric.capitalize()))
        if (index1 != null) {
            testSpeed1(index1, metric, nLoop, nResult, size)
        }
        println()
        val index2 = ANNIndex(dim, path, com.spotify.annoy.IndexType.valueOf(metric.uppercase()))
        testSpeed2(index2, metric, nLoop.toInt(), nResult, size)
        println()
    }
}

fun testSpeed1(index: IAnnoyIndex, metric: String, nLoop: Long, nResult: Int, size: Int) {
    val tStart = System.currentTimeMillis()
    for (i in 0..nLoop) {
        val id = i % size
        val v = index.getItemVector(id)
        index.getNearest(v, nResult, -1, true)
    }
    val tEnd = System.currentTimeMillis()
    val diff = (tEnd - tStart).toDouble()
    println("[Java]com.github.hanabi1224:RuAnnoy")
    println("[$metric] Total time elapsed: ${diff/1000}s")
    println("[$metric] Avg time elapsed: ${diff/nLoop