import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riscalar_gui/features/statistics/models/category_stats.dart';
import 'package:syncfusion_flutter_charts/charts.dart';

class CategoryStatsDoughnut extends ConsumerWidget {
  const CategoryStatsDoughnut(
    this.statistics,
    this.title,
    this.totalLabel, {
    super.key,
  });
  final CategoryStats statistics;
  final String totalLabel;
  final String title;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final data = statistics.classBreakdown.entries
        .map((e) => Stat(e.key, e.value.toDouble()))
        .toList();

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        SfCircularChart(
          title: ChartTitle(
            text: title,
            textStyle: Theme.of(context).textTheme.titleLarge,
          ),
          series: <DoughnutSeries<Stat, String>>[
            DoughnutSeries<Stat, String>(
              explode: false,
              explodeOffset: '10%',
              explodeAll: true,
              dataSource: data,
              // startAngle: 270,
              // endAngle: 90,
              pointColorMapper: (Stat data, _) => data.color,
              xValueMapper: (Stat data, _) => data.key,
              yValueMapper: (Stat data, _) => data.value,
              dataLabelMapper: (Stat data, _) => '${data.key} '
                  '${(data.value / statistics.total * 100).round()}%',
              dataLabelSettings: const DataLabelSettings(
                isVisible: true,
                labelPosition: ChartDataLabelPosition.outside,
                connectorLineSettings: ConnectorLineSettings(
                  type: ConnectorType.curve,
                  length: '20%',
                ),
              ),
            )
          ],
        ),
        Text(
          '$totalLabel: ${statistics.total}',
          style: Theme.of(context).textTheme.bodyMedium,
        ),
      ],
    );
  }
}

class Stat {
  Stat(this.key, this.value, [this.color]);
  final String key;
  final double value;
  final Color? color;
}
