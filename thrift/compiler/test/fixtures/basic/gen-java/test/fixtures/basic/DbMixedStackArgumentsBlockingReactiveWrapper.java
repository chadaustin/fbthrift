/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package test.fixtures.basic;

import com.facebook.thrift.client.*;
import java.util.*;

public class DbMixedStackArgumentsBlockingReactiveWrapper 
    implements DbMixedStackArguments.Reactive {
    private final DbMixedStackArguments _delegate;

    public DbMixedStackArgumentsBlockingReactiveWrapper(DbMixedStackArguments _delegate) {
        
        this._delegate = _delegate;
    }

    @java.lang.Override
    public void dispose() {
        _delegate.close();
    }

    @java.lang.Override
    public reactor.core.publisher.Mono<byte[]> getDataByKey0(final String key) {
        return reactor.core.publisher.Mono.fromSupplier(() -> {
                try {
                    return _delegate.getDataByKey0(key);
                } catch (Throwable _e) {
                    throw reactor.core.Exceptions.propagate(_e);
                }
            }).subscribeOn(com.facebook.thrift.util.resources.RpcResources.getOffLoopScheduler());
    }

    @java.lang.Override
    public reactor.core.publisher.Mono<byte[]> getDataByKey1(final String key) {
        return reactor.core.publisher.Mono.fromSupplier(() -> {
                try {
                    return _delegate.getDataByKey1(key);
                } catch (Throwable _e) {
                    throw reactor.core.Exceptions.propagate(_e);
                }
            }).subscribeOn(com.facebook.thrift.util.resources.RpcResources.getOffLoopScheduler());
    }

}