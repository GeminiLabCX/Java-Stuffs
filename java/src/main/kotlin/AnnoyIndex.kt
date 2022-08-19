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
            nResult: Int,
            searchK: Int,
            shouldIncludeDistance: Boolean
    ): AnnoyIndexSearchResult
}

public class AnnoyIndex(
        val pointer: Long,
        override val dimension: Int,
        override val type: IndexType,
        override val size: Long
) : IAnnoyIndex {
    public override fun getItemVector(itemIndex: Long): FloatArray {
        return NativeMethods.getItemVector(this.pointer, itemIndex)
    }

    public override fun getNearest(
            queryVector: FloatArray,
            nResult: Int,
            searchK: Int,
            shouldIncludeDistance: Boolean
    ): AnnoyIndexSearchResult {
        var idList = LongArray(nResult)
        var distanceList = FloatArray(if (shouldIncludeDistance) nResult else 0)
        val count =
                NativeMethods.getNearest(
                        this.pointer,
                        queryVector,
                        nResult,
                        searchK,
                        shouldIncludeDistance,
                        idList,
                        distanceList)
        if (count < nResult) {
            idList = idList.take(count).toLongArray()
            if (shouldIncludeDistance) {
                distanceList = distanceList.take(count).toFloatArray()
            }
        }
        return AnnoyIndexSearchResult(count, shouldIncludeDistance, idList, distanceList)
    }

    public override fun getNearestToItem(
            itemIndex: Long,
            nResult: Int,
            searchK: Int,
            sho