/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package com.github.hanabi1224.RuAnnoy

import java.io.Closeable

public enum class IndexType(val value: Byte) {
    Angular(0),
    Euclidean(1),
    Manhattan(2),
    Hamming(3),
    Dot(4),
}

public class AnnoyIndexSearchResult(
        val count: Int,
        val distanceIncluded: Boolean,
        val idList: LongArray,
        val distanceList: FloatArray
) {}

public interface IAnnoyIndex : Closeable {
    val dimension: Int
    val type: IndexType
    val size: Long
    fun getItemVector(itemIndex: Long): FloatArray
    fun getNearest(
            queryVector: FloatArray,
            nResult: Int,
            searchK: Int,
            shouldIncludeDistance: Boolean
    ): AnnoyIndexSearchResult
    fun getNearestToItem(
            itemIndex: Long,
            nRe