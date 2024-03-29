use std::{ops::RangeInclusive, time::Instant};
use ethers_core::types::BlockNumber;
use utils::event_emitters::EventEmitters;
use db::interfaces;
use super::{
  static_files_generator_event::StaticFilesGeneratorEvent,
  static_files_handler::StaticFilesHander,
  block_headers_segment::BlockHeadersSegment, segment::Segment
};
use rayon::prelude::*;

pub struct StaticFilesGenerator<Db> {
  staticFilesHandler: StaticFilesHander,
  eventEmitters: EventEmitters<StaticFilesGeneratorEvent>
}

impl<Db> StaticFilesGenerator<Db>
  where
    Db: interfaces::db::Db
{
  pub fn run(&mut self, blockRangeOfSegments: BlockRangeOfSegments) {
    self.eventEmitters.emit(StaticFilesGeneratorEvent::Started { blockRangeOfSegments });

    let generatorStartTime= Instant::now( );

    let mut segments= Vec::<(Box<dyn Segment>, RangeInclusive<BlockNumber>)>::new( );

    if let Some(blockRange)= blockRangeOfSegments.headers {
      segments.push((Box::new(BlockHeadersSegment { }), blockRange));}
    if let Some(blockRange)= blockRangeOfSegments.transactions {
      segments.push((Box::new(BlockHeadersSegment { }), blockRange));}
    if let Some(blockRange)= blockRangeOfSegments.transactionReceipts {
      segments.push((Box::new(BlockHeadersSegment { }), blockRange));}

    segments.par_iter( ).try_for_each(
      |(segment, blockRange)| -> Result<( ), ( )> {

        // TODO: Track time taken for static files generation for each specific segment as metrics.

        Ok(( ))
      }
    );

    unimplemented!( );

    self.eventEmitters.emit(StaticFilesGeneratorEvent::Finished {
      blockRangeOfSegments,
      timeTaken: generatorStartTime.elapsed( )
    });
  }
}

// Specifies the block range for each segment.
pub struct BlockRangeOfSegments {
  headers: Option<RangeInclusive<BlockNumber>>,
  transactions: Option<RangeInclusive<BlockNumber>>,
  transactionReceipts: Option<RangeInclusive<BlockNumber>>
}